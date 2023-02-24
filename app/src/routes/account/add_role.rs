use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use actix_web_grants::proc_macro::has_permissions;
use dashmap::DashSet;
use sea_orm::DbConn;
use sailfish::TemplateOnce;
use validator::{Validate, ValidateArgs, ValidationError};

use crate::db::*;
use crate::auth::Client;
use crate::permissions::PermissionsCollection;
use crate::utils::{e500, see_other, DbErrbExt, ValidationErrorsExt};
use crate::components::Link;
use crate::components::navbar::{NavBar, NavBarBuilder};
use crate::components::titlebar::{TitleBar, TitleBarBuilder};
use crate::domain::role_form::RoleForm;


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
#[post("/roles/add")]
#[has_permissions("roles_create")]
pub async fn add_role(form: web::Form<RoleForm>, db: web::Data<DbConn>, perms: web::Data<PermissionsCollection>) -> Result<impl Responder, actix_web::Error> { 
    let res = form.validate_args(&perms.user_collections);
    if let Err(ref e) = res {
        if e.is_field_invalid("name") { 
            FlashMessage::error("Cannot Add Role - Invalid Name").send();
            return Ok(see_other("/account/roles"));
        }
    }

    res.map_err(e500)?;

    let res = insert_role_with_permissions(&db, form.into_inner()).await;
    if let Err(ref e) = res {
        if e.is_unique_key_constraint() {
            FlashMessage::error("Cannot Add Role - Duplicate Name Found").send();
            return Ok(see_other("/account/roles"));
        }
    }

    res.map_err(e500)?;
    
    // TODO: change flash message type for success
    FlashMessage::error("Role Added").send();
    Ok(see_other("/account/roles"))
}