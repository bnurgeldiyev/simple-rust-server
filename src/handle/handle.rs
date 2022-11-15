use serde::{Serialize, Deserialize};
use crate::model::db::Db;
use crate::model::todo::{TodoUser, User, UserRequest};
use uuid::Uuid;

extern crate bcrypt;

use bcrypt::{hash, verify};
use crate::handle::token::generate_access_token;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuthResponse {
    access_token: String,
    refresh_token: String,
}

impl UserAuthResponse {
    fn init() -> UserAuthResponse {
        UserAuthResponse {
            access_token: String::from(""),
            refresh_token: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreate {
    username: String,
    password: String,
    firstname: String,
    lastname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HandleNotFound {
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralResponse<T> {
    pub status: bool,
    pub data: Option<T>,
}

pub async fn handle_user_auth(body_str: &String, db: &Db) -> UserAuthResponse {

    let des: UserAuth = serde_json::from_str(&body_str).unwrap();

    let user_by_username = match TodoUser::get_by_username(des.username.clone(), &db).await {
        Ok(res) => {
            res
        },
        Err(err) => {
            panic!("Error, {:?}", err)
        }
    };

    let valid = verify(des.password.clone(), &user_by_username.password).unwrap();
    let mut response: UserAuthResponse = UserAuthResponse::init();

    if valid {
        let access_token = generate_access_token(&des.username);
        response.access_token = access_token;

        let refresh_token = Uuid::new_v4().to_string();
        response.refresh_token = refresh_token;

    } else {
        println!("Invalid");
    }

    response
}

pub async fn handle_user_create(body_str: &String, db: &Db) -> UserCreate {
    let des: UserCreate = serde_json::from_str(&body_str).unwrap();
    let hashed = hash(des.password.clone(), 12).unwrap();

    let user_create: UserRequest = UserRequest {
        username: des.username.clone(),
        password: hashed,
        firstname: des.firstname.clone(),
        lastname: des.lastname.clone(),
    };

    let _user_response = match TodoUser::create(&db, user_create).await {
        Ok(res) => {
            res
        },
        Err(err) => {
            panic!("Error, {:?}", err)
        }
    };

    des
}

pub async fn handle_user_get(id: i32, db: &Db) -> Result<GeneralResponse<User>, sqlx::Error> {
    let user = match TodoUser::get(id, &db).await {
        Ok(res) => {
            res
        },
        Err(err) => {
            return Err(err)
        }
    };

    let resp = GeneralResponse {
        status: true,
        data: Some(user),
    };

    Ok(resp)
}

pub async fn handle_user_list(db: &Db) -> Result<GeneralResponse<Vec<User>>, sqlx::Error> {
    let user_list = match TodoUser::list(&db).await {
        Ok(resp) => {
            resp
        },
        Err(err) => {
            panic!("Error, {:?}", err)
        }
    };

    let resp = GeneralResponse {
        status: true,
        data: Some(user_list),
    };

    Ok(resp)
}
