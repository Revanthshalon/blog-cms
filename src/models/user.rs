use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
}
