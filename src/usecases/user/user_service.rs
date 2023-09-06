use std::sync::Arc;
use async_trait::async_trait;
use rocket::http::Status;
use crate::application::auth::login_cqrs::{SignUpCommand, SignUpResponse};
use crate::domain::user::repository::port::UserRepositoryPort;
use crate::port::user::UserModel;
use crate::domain::user::entity::roles::UserRoles;
use crate::domain::user::entity::user::User;
use crate::infrastructure::database::connection::establish_connection;
use crate::persistence::user::repository::user_repository::UserRepository;

#[async_trait]
pub(crate) trait UserServicePort {
    async fn register_user(&self, sign_up_request: SignUpCommand) -> Result<SignUpResponse, Status>;
}

pub(crate) struct UserService {
    pub user_repository: Arc<UserRepository>,
}

impl UserService {
    pub(crate) async fn new() -> UserService {
        let pool = establish_connection().await;
        UserService {user_repository: Arc::new(UserRepository::new(pool))}
    }
}

#[async_trait]
impl UserServicePort for UserService
{
    async fn register_user(&self, sign_up_request: SignUpCommand) -> Result<SignUpResponse, Status> {
        let user = User::new(sign_up_request.username, sign_up_request.email, sign_up_request.password, vec![UserRoles::User]).await;
        let result = self.user_repository.save_user(&user).await;
        if result.is_err() {
            return Err(Status::InternalServerError);
        } else {
            let response = SignUpResponse {
                success: true,
                message: user.id.to_string(),
            };

            Ok(response)
        }
    }
}