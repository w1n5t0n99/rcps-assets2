use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use sea_orm::DbConn;
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use crate::db::*;
use crate::auth::Client;
use crate::utils::e500;
use sailfish::TemplateOnce;
use crate::components::Link;
use crate::components::navbar::{NavBar, NavBarBuilder};
use crate::components::searchbar::{SearchBar, SearchBarBuilder};


#[derive(TemplateOnce)]
#[template(path = "asset_items.stpl")]
struct ListPage<'a> {
    pub messages: Vec<&'a str>,
    pub navbar: NavBar,
    pub search_bar: SearchBar,
}

#[get("/asset_items")]
#[has_permissions("item_view")]
pub async fn get_asset_items(client: web::ReqData<Client>, flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    //tracing::event!(tracing::Level::INFO, "###### asset route called #######");
    let client = client.into_inner();
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

    let navbar = NavBarBuilder::default()
        .sign_out_url(client.url_to("sign_out"))
        .username(&client.name)
        .email(&client.email)
        .is_admin(client.has_permission("edit_settings"))
        .add_link(Link::Active { name: "Asset-Items".into(), url: "/groups/asset_items".into() })
        .add_link(Link::Disabled { name: "User-Items".into(), url: "#".into() })
        .add_link(Link::Disabled { name: "Locations".into(), url: "#".into() })
        .build()
        .map_err(e500)?;

    let add_link = if client.has_permission("item_create") { Link::create_normal("Add", "#") } else { Link::create_disabled("Add", "#") };
    let upload_link = if client.has_permission("item_create") { Link::create_disabled("Upload", "#") } else { Link::create_disabled("Upload", "#") };

    let search_bar = SearchBarBuilder::default()
        .title("Assets".to_string())
        .form_url("/groups/asset_items".to_string())
        .search_filter((None, vec!["all".to_string(), "assets".to_string(), "model".to_string(), "serial #".to_string()]))
        .add_link(add_link)
        .add_link(upload_link)
        .build()
        .map_err(e500)?;
      
    let body = ListPage {
        messages,
        navbar,
        search_bar,
    }
    .render_once()
    .map_err(e500)?;
   
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}