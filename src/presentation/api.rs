use rocket::http::Status;
use rocket::response::content::Json;
use crate::application::auth::login_cqrs::{LoginRequest, LoginResponse, SignUpRequest, SignUpResponse};

#[post("/login", data = "<login_request>")]
pub async fn login(login_request: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    // Call the application service to authenticate the user and generate a JWT
}

pub async fn sign_up(sign_up_request: Json<SignUpRequest>) -> Result<Json<SignUpResponse>, Status> {
    // Call the application service to create a new user
}