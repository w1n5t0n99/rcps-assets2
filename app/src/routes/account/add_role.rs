use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use actix_web_grants::proc_macro::has_permissions;
use dashmap::DashSet;
use futures::TryFutureExt;
use sea_orm::DbConn;
use crate::db::*;
use crate::auth::Client;
use crate::permissions::PermissionsCollection;
use crate::utils::{e500, see_other, DbErrbExt, e400};
use sailfish::TemplateOnce;
use crate::components::Link;
use crate::components::navbar::{NavBar, NavBarBuilder};
use crate::components::titlebar::{TitleBar, TitleBarBuilder};
use validator::{Validate, ValidateArgs, ValidationError};

use ::entity::{roles};
use ::entity::prelude::{Roles};


#[derive(TemplateOnce)]
#[template(path = "add-role.stpl")]
struct ListPage {
    pub navbar: NavBar,
    pub titlebar: TitleBar,
}

#[get("/roles/add")]
#[has_permissions("roles_create")]
pub async fn add_role_form(client: web::ReqData<Client>) -> Result<impl Responder, actix_web::Error> {
    let client = client.into_inner();

    let navbar = NavBarBuilder::default()
        .sign_out_url(client.url_to("sign_out"))
        .username(&client.name)
        .email(&client.email)
        .is_admin(client.has_permission("edit_settings"))
        .add_link(Link::Normal { name: "Asset-Items".into(), url: "/groups/asset_items".into() })
        .add_link(Link::Disabled { name: "User-Items".into(), url: "#".into() })
        .add_link(Link::Disabled { name: "Locations".into(), url: "#".into() })
        .build()
        .map_err(e500)?;

    let titlebar = TitleBarBuilder::default()
        .title("Add New Role".to_string())
        .links(Vec::new())
        .build()
        .map_err(e500)?;

    let body = ListPage {
            navbar,
            titlebar,
        }
        .render_once()
        .map_err(e500)?;
   
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

//=================================================================
fn validate(value: &String, arg: &DashSet<String>) -> Result<(), ValidationError> {
    if arg.contains(value) == false {
        return Err(ValidationError::new("valid permission not found"));
    }

    Ok(())
}

#[derive(serde::Deserialize, Validate)]
pub struct FormData {
    #[validate(length(min = 1))]
    name: String,
    description: String,
    #[validate(custom(function = "validate", arg = "&'v_a DashSet<String>"))]
    perm0: Option<String>,
    #[validate(custom(function = "validate", arg = "&'v_a DashSet<String>"))]
    perm1: Option<String>,
    #[validate(custom(function = "validate", arg = "&'v_a DashSet<String>"))]
    perm2: Option<String>,
    #[validate(custom(function = "validate", arg = "&'v_a DashSet<String>"))]
    perm3: Option<String>,
    #[validate(custom(function = "validate", arg = "&'v_a DashSet<String>"))]
    perm4: Option<String>,
    #[validate(custom(function = "validate", arg = "&'v_a DashSet<String>"))]
    perm5: Option<String>,
    #[validate(custom(function = "validate", arg = "&'v_a DashSet<String>"))]
    perm6: Option<String>,
    #[validate(custom(function = "validate", arg = "&'v_a DashSet<String>"))]
    perm7: Option<String>,
}

impl FormData {
    pub fn get_permissions(&self) -> Vec<String> {
        let mut perms = Vec::with_capacity(8);

        if let Some(ref p) = self.perm0 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm1 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm2 {
            perms.push(p.clone());

        }
        if let Some(ref p) = self.perm3 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm4 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm5 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm6 {
            perms.push(p.clone());
        }
        
        if let Some(ref p) = self.perm7 {
            perms.push(p.clone());
        }

        perms
    }
}

#[post("/roles/add")]
#[has_permissions("roles_create")]
pub async fn add_role(form: web::Form<FormData>, db: web::Data<DbConn>, perms: web::Data<PermissionsCollection>) -> Result<impl Responder, actix_web::Error> { 
    if form.validate_args((
        &perms.user_collections,
        &perms.user_collections,
        &perms.user_collections,
        &perms.user_collections,
        &perms.user_collections,
        &perms.user_collections,
        &perms.user_collections,
        &perms.user_collections,
    )).is_err() {
        FlashMessage::error("Invalid form data.").send();
        return Ok(see_other("/account/roles"));
    }

    let perms = form.get_permissions();
    insert_role_with_permissions(&db, form.name.clone(), form.description.clone(), perms)
        .await
        .map_err(|e| {
            if e.is_unique_key_constraint() {FlashMessage::error("Duplicate roles found").send(); e400(e) }
            else { e500(e) }
        })?;
    
    // TODO: change flash message type for success
    FlashMessage::error("Role added").send();
    Ok(see_other("/account/roles"))
}