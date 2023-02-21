pub mod navbar;
pub mod searchbar;


#[derive(Debug, Clone)]
pub enum Link {
    Active {name: String, url: String},
    Disabled {name: String, url: String},
    Normal {name: String, url: String},
}

impl Link {
    pub fn create_active<S: Into<String>>(name: S, url: S) -> Link {
        Link::Active { name: name.into(), url: url.into() }
    }

    pub fn create_disabled<S: Into<String>>(name: S, url: S) -> Link {
        Link::Disabled { name: name.into(), url: url.into() }
    }

    pub fn create_normal<S: Into<String>>(name: S, url: S) -> Link {
        Link::Normal { name: name.into(), url: url.into() }
    }
}