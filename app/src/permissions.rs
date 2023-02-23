use dashmap::DashSet;


#[derive(Clone, Debug)]
pub struct PermissionsCollection {
    pub user_collections: DashSet<String>,
    pub admin_collections: DashSet<String>,
}

impl PermissionsCollection {
    pub fn create_collection() -> Self {
        let user_collections = DashSet::with_capacity(12);
        user_collections.insert("item_assign_user".to_string());
        user_collections.insert("item_assign_location".to_string());
        user_collections.insert("item_view".to_string());
        user_collections.insert("item_edit".to_string());
        user_collections.insert("item_create".to_string());
        user_collections.insert("item_delete".to_string());
        user_collections.insert("profile_view".to_string());
        user_collections.insert("profile_edit".to_string());

        let admin_collections = DashSet::with_capacity(6);
        admin_collections.insert("user_view".to_string());
        admin_collections.insert("user_edit".to_string());
        admin_collections.insert("user_create".to_string());
        admin_collections.insert("user_delete".to_string());
        admin_collections.insert("view_settings".to_string());
        admin_collections.insert("edit_settings".to_string());

        PermissionsCollection {
            user_collections,
            admin_collections,
        }
    }

    pub fn contains_user_permission<S: Into<String>>(&self, perm: S) -> bool {
        self.user_collections.contains(&perm.into())
    }

    pub fn contains_admin_permission<S: Into<String>>(&self, perm: S) -> bool {
        self.admin_collections.contains(&perm.into())
    }
}