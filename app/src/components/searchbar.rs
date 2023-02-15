use super::Link;


#[derive(Debug, derive_builder::Builder)]
//#[builder(build_fn(validate = "Self::validate"))]
pub struct SearchBar {
    pub title: String,
    pub form_url: String,
    #[builder(setter(into, strip_option), default)]
    pub search_text: Option<String>,
    // selected item, list of items
    #[builder(setter(strip_option), default)]
    pub search_filter: Option<(Option<String>,Vec<String>)>,
    #[builder(setter(each(name = "add_link")))]
    pub links: Vec<Link>,
}





