use super::Link;

#[derive(Debug, derive_builder::Builder)]
#[builder(setter(into))]
pub struct NavBar {
    pub username: String,
    pub email: String,
    #[builder(setter(each(name = "add_link")))]
    pub links: Vec<Link>,
    pub is_admin: bool,
    pub sign_out_url: String,
}

