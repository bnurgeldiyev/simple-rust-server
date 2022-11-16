use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct UserBase {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct UserGetByUsername {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Default, Clone)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl UserAuthResponse {
    pub fn init() -> UserAuthResponse {
        UserAuthResponse {
            access_token: String::from(""),
            refresh_token: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}
