use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum UserRoles {
    User,
    Admin,
    SuperAdmin,
}

impl FromStr for UserRoles {
    type Err = ();

    fn from_str(s: &str) -> Result<UserRoles, Self::Err> {
        match s {
            "User" => Ok(UserRoles::User),
            "Admin" => Ok(UserRoles::Admin),
            "SuperAdmin" => Ok(UserRoles::SuperAdmin),
            _ => Err(()),
        }
    }
}