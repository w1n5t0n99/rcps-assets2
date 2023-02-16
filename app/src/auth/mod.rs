mod password;
mod middleware;
mod client;

pub use password::{change_password, validate_credentials, AuthError, Credentials};
pub use middleware::reject_anonymous_users;
pub use middleware::reject_anonymous_and_invalid_users;
pub use middleware::extract_user_roles;
pub use client::{Client, ClientError};