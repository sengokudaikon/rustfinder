use std::error::Error;
use anyhow::anyhow;
use tokio_postgres::{Client, NoTls};
use crate::domain::user::repository::user_repository_port::UserRepositoryPort;
use crate::domain::user::user_model::User;

pub struct UserRepository {
    client: Client
}

impl UserRepositoryPort for UserRepository {
    async fn find_user_by_username(&self, username: &str) -> Result<User, dyn Error> {
        let row = self.client.query_opt("SELECT id, username, roles FROM users WHERE username = $1", &[&username]).await?;

        if let Some(row) = row {
            let user = User {
                id: row.get(0),
                username: row.get(1),
                roles: row.get(2),
            };
            Ok((user))
        } else {
            Err(anyhow!("User not found").into())
        }
    }

    async fn new() -> Result<Self, dyn Error> {
        let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=your_password dbname=your_database", NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Self { client })
    }
}