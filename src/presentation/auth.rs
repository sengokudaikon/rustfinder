use std::sync::Arc;
use rocket::{post, State};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::usecases::user::auth_service::{AuthService, AuthServicePort};
use crate::usecases::user::user_service::{UserService, UserServicePort};
use crate::application::auth::login_cqrs::{LoginCommand, LoginResponse, SignUpCommand, SignUpResponse};


#[post("/login", format = "application/json", data = "<login_request>")]
pub async fn login(controller: &State<AuthController>, login_request: Json<LoginCommand>) -> Result<Json<LoginResponse>, Status> {
    let login_result = controller.authenticate_user_login(login_request).await;
    match login_result {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/signUp", format = "application/json", data = "<sign_up_request>")]
pub async fn sign_up(controller: &State<AuthController>, sign_up_request: Json<SignUpCommand>) -> Result<Json<SignUpResponse>, Status> {
    let sign_up_result = controller.authenticate_user_sign_up(sign_up_request).await;
    match sign_up_result {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(Status::InternalServerError),
    }
}
pub(crate) struct AuthController {
    auth_service: Arc<AuthService>,
    user_service: Arc<UserService>,
}

impl AuthController {
    pub(crate) async fn new() -> AuthController {
        let auth_service = Arc::new(AuthService::new().await);
        let user_service = Arc::new(UserService::new().await);
        AuthController {auth_service, user_service}
    }

    pub async fn authenticate_user_login(&self, login_request: Json<LoginCommand>) -> Result<LoginResponse, Status> {
        let response = self.auth_service.authenticate_user(login_request.0).await;
        if response.is_err() {
            return Err(Status::InternalServerError);
        } else {
            Ok(response.unwrap())
        }
    }

    pub async fn authenticate_user_sign_up(&self, sign_up_request: Json<SignUpCommand>) -> Result<SignUpResponse, Status> {
        let response = self.user_service.register_user(sign_up_request.0).await;
        if response.is_err() {
            return Err(Status::InternalServerError);
        } else {
            Ok(response.unwrap())
        }
    }
}