pub mod response;
pub mod body;

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum RoleModel {
    Admin,
    Manager,
    Member,
}