use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

use actix_web::{post, HttpResponse};
use actix_web_flash_messages::FlashMessage;


#[post("/sign_out")]
pub async fn sign_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        Ok(see_other("/user/sign_in"))
    } else {
        session.log_out();
        FlashMessage::info("You have successfully logged out.").send();
        Ok(see_other("/user/sign_in"))
    }
}