use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use sea_orm::DbConn;
use crate::db::*;
use crate::auth::Client;
use crate::permissions::PermissionsCollection;
use crate::utils::{e500, see_other};
use sailfish::TemplateOnce;
use crate::components::Link;
use crate::components::navbar::{NavBar, NavBarBuilder};
use crate::components::titlebar::{TitleBar, TitleBarBuilder};
use std::fmt;
use serde::de::{self, value, Deserialize, Deserializer, Visitor, SeqAccess};

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

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    description: String,
    #[serde(deserialize_with = "string_or_vec")]
    perms: Vec<String>,
}

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    struct StringOrVec;

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where E: de::Error
        {
            Ok(vec![s.to_owned()])
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where S: SeqAccess<'de>
        {
            Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrVec)
}

#[post("/roles/add")]
#[has_permissions("roles_create")]
pub async fn add_role(form: web::Form<FormData>, db: web::Data<DbConn>, perms: web::Data<PermissionsCollection>) -> Result<impl Responder, actix_web::Error> { 

    Ok(see_other("/account/roles"))
}