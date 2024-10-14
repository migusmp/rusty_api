use serde::{Deserialize, Serialize};

use crate::utils::jwt::generate_token;

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub exp: String,
    pub iat: String,
}

impl RegisterUser {
    pub fn new(username: String, email: String, password: String) -> RegisterUser {
        RegisterUser {
            username,
            email,
            password,
        }
    }
}

impl LoginUser {
    pub fn new(username: String, password: String) -> Self {
        LoginUser { username, password }
    }
}

impl Payload {
    pub fn new(
        id: String,
        name: String,
        email: String,
        password: String,
        created_at: String,
        exp: String,
        iat: String,
    ) -> Self {
        Payload {
            id,
            name,
            email,
            password,
            created_at,
            exp,
            iat,
        }
    }

    pub fn token(&Self) -> String {
       let token = generate_token(Self);
        Ok(token)
    }
}
