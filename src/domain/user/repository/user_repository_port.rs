use std::error::Error;
use tokio_postgres::Client;
use crate::domain::user::user_model::User;
use crate::persistence::user::repository::user_repository::UserRepository;

pub trait UserRepositoryPort {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, dyn Error>;
    // Add other methods as needed
    fn new() -> Result<Self, dyn Error>;
}
