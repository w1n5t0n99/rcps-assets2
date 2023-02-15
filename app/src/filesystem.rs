use std::{path::PathBuf, collections::HashMap, fs::File};
use actix_multipart::{Multipart, Field};
use futures::{TryStreamExt, StreamExt};
use mime::Mime;
use anyhow::{anyhow, Context};
use std::io::Write;

use crate::utils::{error_chain_fmt, spawn_blocking_with_tracing};



#[derive(thiserror::Error)]
pub enum FileSystemError {
    #[error("Error processing payload")]
    Payload(#[source] anyhow::Error),
    #[error("Something went wrong")]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Clone)]
pub enum UploadPayload {
    File(FilePayload),
    Nonfile(NonfilePayload),
}

#[derive(Clone)]
pub struct FilePayload {
    pub data: Vec<u8>,
    pub filename: String,
    pub hash: blake3::Hash,
    pub tmp_path: PathBuf,
    pub mime: Mime,
}

#[derive(Clone)]
pub struct NonfilePayload {
    pub data: Vec<u8>,
    pub text: String,
}

pub async fn process_multipart_fields(mut mutipart: Multipart) -> Result<HashMap<String, UploadPayload>, FileSystemError> {
    let mut payload_map = HashMap::new();

    while let Ok(Some(mut field)) = mutipart.try_next().await {
        let name = field.content_disposition()
            .get_name()
            .context("Field name not found")
            .map_err(|e| FileSystemError::Payload(anyhow!(e)))?
            .to_string();

        let mime = field.content_type();
        match mime {
            None => {
                let f = process_nonfile_field(&mut field).await?;
                match f {
                    Some(f) => { payload_map.insert(name, UploadPayload::Nonfile(f)); }
                    None => { }
                }
            }
            Some(_) => {
                let f = process_file_field(&mut field).await?;
                match f {
                    Some(f) => { payload_map.insert(name, UploadPayload::File(f)); }
                    None => { }
                }
            }
        }
    }

    Ok(payload_map)
}

// If the text field parsing fails returns Error
// If the field is empty or can't be converted to valid utf8 return None
async fn process_nonfile_field(field: &mut Field) -> Result<Option<NonfilePayload>, FileSystemError> {

    let mut data: Vec<u8> = Vec::with_capacity(1024);
    while let Some(chunk) = field.next().await {
        let byte = chunk.map_err(|e| FileSystemError::Payload(anyhow!(e)))?;
        data.extend(byte.to_owned());
    }

    if data.is_empty() {
        return Ok(None);
    }

    let text = match std::str::from_utf8(&data) {
        Ok(text) => text.to_string(),
        Err(e) => { return Ok(None); }
    };

    Ok(Some(NonfilePayload { data, text } ))
}

// If the file field parsing fails returns Error
// If the field is empty return None
async fn process_file_field( field: &mut Field) -> Result<Option<FilePayload>, FileSystemError> {

    let filename = field
        .content_disposition()
        .get_filename()
        .context("field missing filename")
        .map_err(|e| FileSystemError::Payload(e))?
        .to_string();

    let mime = field
        .content_type()
        .context("field mime not found")
        .map_err(|e| FileSystemError::Payload(e))?
        .clone();

    let ext = mime2ext::mime2ext(&mime)
        .context("Could not guess fiel field ext")
        .map_err(|e| FileSystemError::Payload(e))?;

    let tmp_path = format!("./app/tempfiles/{}.{}", uuid::Uuid::new_v4(), ext);

    let mut hasher = blake3::Hasher::new();
    let mut data: Vec<u8> = Vec::with_capacity(1024);
    while let Some(chunk) = field.next().await {
        let bytes = chunk.map_err(|e| FileSystemError::Payload(anyhow!(e)))?;

        hasher.update(&bytes);
        data.extend(bytes.to_owned());
    }

    if data.is_empty() {
        return Ok(None);
    }

    let payload = FilePayload { 
        data: data,
        filename: filename.to_string(),
        hash: hasher.finalize(),
        tmp_path: tmp_path.into(),
        mime: mime 
    };

    save_payload_to_temp_file(&payload).await?;

    Ok(Some(payload))
}

async fn save_payload_to_temp_file(payload: &FilePayload) -> Result<(), FileSystemError> {
    // create temporary file
    let tmp_path = payload.tmp_path.clone();
    let mut f = spawn_blocking_with_tracing(move || {
        File::create(tmp_path)
    })
    .await
    .context("Blocking thread error")?
    .context("Could not create tmp file")?;
    
    // filesystem operations are blocking
    let data = payload.data.clone();
    spawn_blocking_with_tracing(move|| f.write_all(&data))
        .await
        .context("Blocking thread error")?
        .context("Could write data to tmp file")?;
    

    Ok(())
}
