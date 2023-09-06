use rocket::serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpCommand {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpResponse {
    pub success: bool,
    pub message: String,
}