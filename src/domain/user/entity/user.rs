use serde::{Deserialize, Serialize};
use rocket::serde::uuid::Uuid;
use std::fmt::Debug;
use crate::domain::user::entity::roles::UserRoles;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<UserRoles>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub roles: Vec<UserRoles>,
}