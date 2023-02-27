use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use actix_web_grants::proc_macro::has_permissions;
use dashmap::DashSet;
use sea_orm::DbConn;
use sailfish::TemplateOnce;
use validator::{Validate, ValidateArgs, ValidationError, validate_must_match};

use crate::db;
use crate::auth::Client;
use crate::permissions::PermissionsCollection;
use crate::utils::{e500, see_other, DbErrbExt, ValidationErrorsExt};
use crate::components::Link;
use crate::components::navbar::{NavBar, NavBarBuilder};
use crate::components::titlebar::{TitleBar, TitleBarBuilder};
use crate::domain::role_form::RoleForm;


#[derive(TemplateOnce)]
#[template(path = "edit-role.stpl")]
struct EditPage {
    pub navbar: NavBar,
    pub titlebar: TitleBar,
    pub name: String,
    pub description: String,
    pub perms: DashSet<String>,
}

#[get("/roles/{id}/edit")]
#[has_permissions("roles_edit")]
pub async fn edit_role_form(path: web::Path<String>, client: web::ReqData<Client>, dbconn: web::Data<DbConn>) -> Result<impl Responder, actix_web::Error> {
    let client = client.into_inner();
    let role_id = path.into_inner(); 

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
        .title("Role Configuration".to_string())
        .links(Vec::new())
        .build()
        .map_err(e500)?;

    let perms = db::find_role_permissions(role_id, &dbconn).await.map_err(e500)?;
    let name = perms.0.id;
    let description = perms.0.description;
    let perms: DashSet<String> = DashSet::from_iter(perms.1);

    let body = EditPage {
            navbar,
            titlebar,
            name,
            description,
            perms,
        }
        .render_once()
        .map_err(e500)?;
   
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

//=================================================================
#[post("/roles/{id}/edit")]
#[has_permissions("roles_edit")]
pub async fn edit_role(form: web::Form<RoleForm>, dbconn: web::Data<DbConn>, perms: web::Data<PermissionsCollection>) -> Result<impl Responder, actix_web::Error> { 
    let res = form.validate_args(&perms.user_collections);
    
    
    // TODO: change flash message type for success
    FlashMessage::error("Role Updated").send();
    Ok(see_other("/account/roles"))
}