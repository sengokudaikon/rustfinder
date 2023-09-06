use rocket::serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginCommand {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToResponse)]
pub struct LoginResponse {
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SignUpCommand {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToResponse)]
pub struct SignUpResponse {
    pub success: bool,
    pub message: String,
}