use async_trait::async_trait;

use crate::domain::user::entity::user::User;
use crate::domain::user::entity::roles::UserRoles;

#[async_trait]
pub trait UserModel {
    async fn new(username: String, email: String, password: String, roles: Vec<UserRoles>) -> User;
}

#[async_trait]
impl UserModel for User {
    async fn new(username: String, email: String, password: String, roles: Vec<UserRoles>) -> User {
        let user = User {
            id: uuid::Uuid::new_v4(),
            username,
            email,
            password,
            roles
        };

        user
    }
}