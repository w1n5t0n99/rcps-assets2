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
use crate::components::Link;
use crate::components::navbar::{NavBar, NavBarBuilder};
use crate::components::searchbar::{SearchBar, SearchBarBuilder};

use ::entity::{roles};
use ::entity::prelude::{Roles};


#[derive(TemplateOnce)]
#[template(path = "roles.stpl")]
struct ListPage<'a> {
    pub messages: Vec<&'a str>,
    pub navbar: NavBar,
    pub search_bar: SearchBar,
    pub roles: Vec<roles::Model>,
}

#[get("/roles")]
#[has_permissions("roles_view")]
pub async fn view_roles(client: web::ReqData<Client>, db: web::Data<DbConn>, flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    let client = client.into_inner();
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

    let navbar = NavBarBuilder::default()
        .sign_out_url(client.url_to("sign_out"))
        .username(&client.name)
        .email(&client.email)
        .is_admin(client.has_permission("edit_settings"))
        .add_link(Link::Active { name: "Asset-Items".into(), url: "/groups/asset_items".into() })
        .add_link(Link::Normal { name: "User-Items".into(), url: "#".into() })
        .build()
        .map_err(e500)?;

    let add_link = if client.has_permission("roles_create") { Link::create_normal("Add", "#") } else { Link::create_disabled("Add", "#") };
    let upload_link = if client.has_permission("roles_create") { Link::create_disabled("Upload", "#") } else { Link::create_disabled("Upload", "#") };

    let search_bar = SearchBarBuilder::default()
        .title("Roles".to_string())
        .form_url("/account/roles".to_string())
        .search_filter((None, vec!["all".to_string()]))
        .add_link(add_link)
        .add_link(upload_link)
        .build()
        .map_err(e500)?;

    let roles = find_roles(&db)
        .await
        .map_err(e500)?;

    let body = ListPage {
            messages,
            navbar,
            search_bar,
            roles,
        }
        .render_once()
        .map_err(e500)?;
   
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}