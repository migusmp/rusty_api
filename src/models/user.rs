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
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub exp: i64,
    pub iat: i64,
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
        id: i64,
        name: String,
        email: String,
        password: String,
        created_at: String,
        exp: i64,
        iat: i64,
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

    pub fn token(&self) -> Result<String, jsonwebtoken::errors::Error> {
        generate_token(self)
    }
}
