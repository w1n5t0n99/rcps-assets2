pub mod navbar;
pub mod searchbar;


#[derive(Debug, Clone)]
pub enum Link {
    Active {name: String, url: String},
    Disabled {name: String, url: String},
    Normal {name: String, url: String},
}