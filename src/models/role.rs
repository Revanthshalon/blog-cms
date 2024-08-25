use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRole {
    pub role_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRole {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRole {
    pub id: Uuid,
    pub role_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRole {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponse {
    pub id: Uuid,
    pub role_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListResponse {
    pub roles: Vec<RoleResponse>,
}
