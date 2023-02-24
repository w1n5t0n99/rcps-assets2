use dashmap::DashSet;
use validator::{Validate, ValidationError};


fn validate_permissions(value: &RoleForm, arg: &DashSet<String>) -> Result<(), ValidationError> {
    
    if let Some(ref v) = value.perm0 {
        if arg.contains(v) == false { return Err(ValidationError::new("valid permission not found")); }
    }
    
    if let Some(ref v) = value.perm1 {
        if arg.contains(v) == false { return Err(ValidationError::new("valid permission not found")); }
    }

    if let Some(ref v) = value.perm2 {
        if arg.contains(v) == false { return Err(ValidationError::new("valid permission not found")); }
    }

    if let Some(ref v) = value.perm3 {
        if arg.contains(v) == false { return Err(ValidationError::new("valid permission not found")); }
    }

    if let Some(ref v) = value.perm4 {
        if arg.contains(v) == false { return Err(ValidationError::new("valid permission not found")); }
    }

    if let Some(ref v) = value.perm5 {
        if arg.contains(v) == false { return Err(ValidationError::new("valid permission not found")); }
    }

    if let Some(ref v) = value.perm6 {
        if arg.contains(v) == false { return Err(ValidationError::new("valid permission not found")); }
    }

    if let Some(ref v) = value.perm7 {
        if arg.contains(v) == false { return Err(ValidationError::new("valid permission not found")); }
    }

    Ok(())
}

#[derive(serde::Deserialize, Validate, Clone)]
#[validate(schema(function = "validate_permissions", arg = "&'v_a DashSet<String>"))]
pub struct RoleForm {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: String,
    pub perm0: Option<String>,
    pub perm1: Option<String>,
    pub perm2: Option<String>,
    pub perm3: Option<String>,
    pub perm4: Option<String>,
    pub perm5: Option<String>,
    pub perm6: Option<String>,
    pub perm7: Option<String>,
}

impl RoleForm {
    pub fn get_permissions_vec(&self) -> Vec<String> {
        let mut perms = Vec::with_capacity(8);

        if let Some(ref p) = self.perm0 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm1 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm2 {
            perms.push(p.clone());

        }
        if let Some(ref p) = self.perm3 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm4 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm5 {
            perms.push(p.clone());
        }

        if let Some(ref p) = self.perm6 {
            perms.push(p.clone());
        }
        
        if let Some(ref p) = self.perm7 {
            perms.push(p.clone());
        }

        perms
    }
}