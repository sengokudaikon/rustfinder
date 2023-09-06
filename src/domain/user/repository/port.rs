use std::sync::Arc;
use async_trait::async_trait;
use deadpool_postgres::{Object, Pool};
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use crate::domain::user::entity::user::User;
use crate::persistence::user::repository::user_repository::UserRepository;

#[async_trait]
pub(crate) trait UserRepositoryPort {
    fn new(pool: Arc<Pool>) -> UserRepository;
    async fn get_client(&self) -> Result<Object, anyhow::Error>;
    async fn execute_query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, anyhow::Error>;
    async fn save_user(&self, user: &User) -> Result<(), anyhow::Error>;
    async fn find_user(&self, criterion: &str, value: &str) -> Result<User, anyhow::Error>;
    async fn find_user_by_email(&self, email: &str) -> Result<User, anyhow::Error>;
    async fn find_user_by_username(&self, username: &str) -> Result<User, anyhow::Error>;
}
