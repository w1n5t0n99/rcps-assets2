use super::Link;


#[derive(Debug, derive_builder::Builder)]
//#[builder(build_fn(validate = "Self::validate"))]
pub struct TitleBar {
    pub title: String,
    #[builder(setter(each(name = "add_link")))]
    pub links: Vec<Link>,
}




