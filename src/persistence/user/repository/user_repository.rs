use std::str::FromStr;
use std::sync::Arc;
use anyhow::anyhow;
use async_trait::async_trait;
use deadpool_postgres::{Object, Pool};
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use crate::domain::user::entity::roles::UserRoles;
use crate::domain::user::entity::user::User;
use crate::port::user::UserModel;
use crate::domain::user::repository::port::UserRepositoryPort;

pub(crate) struct UserRepository {
    pub(crate) pool: Arc<Pool>,
}
#[async_trait]
impl UserRepositoryPort for UserRepository {
    fn new(pool: Arc<Pool>) -> UserRepository {
        UserRepository { pool }
    }
    async fn get_client(&self) -> Result<Object, anyhow::Error> {
        self.pool.get().await.map_err(|e| anyhow!("Failed to get a connection from the pool: {}", e))
    }

    async fn execute_query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, anyhow::Error> {
        let client = self.get_client().await?;
        let statement = client.prepare(query).await?;
        let rows = client.query(&statement, params).await;
        if rows.is_err() {
            return Err(anyhow!("Failed to execute query: {}", rows.err().unwrap()));
        } else {
            Ok(rows.unwrap())
        }
    }

    async fn save_user(&self, user: &User) -> Result<(), anyhow::Error> {
        let query = "INSERT INTO users (id, username, email, roles) VALUES ($1, $2, $3, $4)";
        let roles: String = user.roles.iter().map(|role| match role {
            UserRoles::User => "User",
            UserRoles::Admin => "Admin",
            UserRoles::SuperAdmin => "SuperAdmin",
        }).collect();
        let roles = serde_json::to_string(&roles).unwrap();
        let params: [&(dyn ToSql + Sync); 4] = [&user.id.as_ref(), &user.username, &user.email, &roles];
        self.execute_query(query, &params).await?;
        Ok(())
    }

    async fn find_user(&self, criterion: &str, value: &str) -> Result<User, anyhow::Error> {
        let query = format!("SELECT id, username, email, password, roles FROM users WHERE {} = $1", criterion);
        let params: [&(dyn ToSql + Sync); 1] = [&value];
        let rows = self.execute_query(&query, &params).await?;
        if let Some(row) = rows.get(0) {
            let username: String = row.get(1);
            let email: String = row.get(2);
            let password: String = row.get(3);
            let role_string: String = row.get(4);
            let roles: Vec<UserRoles> = vec![UserRoles::from_str(role_string.as_str()).unwrap()];
            let user = User::new(
                username,
                email,
                password,
                roles,
            ).await;
            Ok(user)
        } else {
            Err(anyhow!("User not found"))
        }
    }

    async fn find_user_by_email(&self, email: &str) -> Result<User, anyhow::Error> {
        self.find_user("email", email).await
    }

    async fn find_user_by_username(&self, username: &str) -> Result<User, anyhow::Error> {
        self.find_user("username", username).await
    }
}