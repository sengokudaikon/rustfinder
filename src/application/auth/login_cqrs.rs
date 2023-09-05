use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpResponse {
    pub success: bool,
    pub message: String,
}