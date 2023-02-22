use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use sea_orm::DbConn;
use actix_web_flash_messages::IncomingFlashMessages;
use crate::db::*;
use crate::auth::Client;
use crate::utils::e500;
use sailfish::TemplateOnce;
use crate::components::{Link, titlebar};
use crate::components::navbar::{NavBar, NavBarBuilder};
use crate::components::titlebar::{TitleBar, TitleBarBuilder};

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
pub async fn add_role_form(client: web::ReqData<Client>, db: web::Data<DbConn>, flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    let client = client.into_inner();
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

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

    let roles = find_roles(&db)
        .await
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