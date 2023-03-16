use oso::{Oso, ToPolar};
use crate::error_responses::{e500, e404, e403};


pub struct Authorize {
    oso: Oso,
}

impl Authorize {
    pub fn new(oso: Oso) -> Self {
        Self {
            oso,
        }
    }

    pub fn is_allowed<Actor, Action, Resource>(&self, actor: Actor, action: Action, resource: Resource) -> Result<bool, actix_web::Error>
    where 
        Actor: ToPolar,
        Action: ToPolar,
        Resource: ToPolar,
    {
        let res = self.oso.is_allowed(actor, action, resource)
            .map_err(|e|  e500("error", "Unexpected server error occured", e))?;
        
        Ok(res)
    }

    pub fn is_allowed_or_forbidden<Actor, Action, Resource>(&self, actor: Actor, action: Action, resource: Resource) -> Result<(), actix_web::Error>
    where 
        Actor: ToPolar,
        Action: ToPolar,
        Resource: ToPolar,
    {
        let res = self.oso.is_allowed(actor, action, resource)
            .map_err(|e|  e500("error", "Unexpected server error occured", e))?;

        if res == false {
            return Err(e403("fail", "User does not have permission", "Forbidden"))
        }
        
        Ok(())
    }

    pub fn is_allowed_or_not_found<Actor, Action, Resource>(&self, actor: Actor, action: Action, resource: Resource) -> Result<(), actix_web::Error>
    where 
        Actor: ToPolar,
        Action: ToPolar,
        Resource: ToPolar,
    {
        let res = self.oso.is_allowed(actor, action, resource)
            .map_err(|e|  e500("error", "Unexpected server error occured", e))?;

        if res == false {
            return Err(e404("fail", "Resource not found", "NotFound"))
        }
        
        Ok(())
    }
}

