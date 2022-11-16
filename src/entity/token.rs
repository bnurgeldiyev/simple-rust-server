use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::BTreeMap;
use actix_web::HttpRequest;
use jwt::{SignWithKey, VerifyWithKey};
use crate::Db;
use crate::model::todo::TodoUser;
use serde::{Serialize};

const TOKEN_SECRET_KEY: &str = "bnurgeldiyev514";
const TOKEN_LIFE_TIME: u64 = 5; // minute

#[derive(Debug, PartialEq)]
pub enum AccessTokenError {
    TokenInvalid,
    ExpiredToken,
}

struct AccessTokenResult {
    username: String,
    remaining_time: u64,
}

#[derive(Debug, Serialize)]
pub struct ActionInfo {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub remaining_time: u64,
}

fn get_time_sec() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}

pub fn generate_access_token(username: &String) -> String {

    let key: Hmac<Sha256> = Hmac::new_from_slice(TOKEN_SECRET_KEY.as_ref()).unwrap();
    let mut claims = BTreeMap::new();
    let start = get_time_sec() + (TOKEN_LIFE_TIME * 60);
    let start_str = start.to_string().clone();
    claims.insert("username", username);
    claims.insert("created_time", &start_str);

    let token_str = claims.sign_with_key(&key).unwrap();

    token_str
}

async fn verify_access_token(access_token: &str) -> Result<AccessTokenResult, AccessTokenError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(TOKEN_SECRET_KEY.as_ref()).unwrap();
    let claims: BTreeMap<String, String> = match access_token.verify_with_key(&key) {
        Ok(res) => {
            res
        },
        Err(_err) => {
            return Err(AccessTokenError::TokenInvalid);
        }
    };

    let token_expire: u64 = claims["created_time"].parse().unwrap();
    if get_time_sec() > token_expire {
        return Err(AccessTokenError::ExpiredToken);
    }

    let remaining_time: u64 = token_expire - get_time_sec();
    let response = AccessTokenResult {
        username: claims["username"].to_string(),
        remaining_time,
    };

    Ok(response)
}

pub async fn is_unauthorized(req: &HttpRequest, db: &Db) -> Result<ActionInfo, AccessTokenError> {
    let token = req.headers().get("authorization").unwrap().to_str().unwrap();
    let token = &token[7..];

    let token_response: AccessTokenResult;
    match verify_access_token(token).await {
        Ok(res) => {
            token_response = res;
        },
        Err(err) => {
            return Err(err);
        }
    };

    let user = match TodoUser::get_by_username(token_response.username, &db).await {
        Ok(res) => {
            res
        },
        Err(_err) => {
            panic!("Error: TodoUser::get_by_username")
        }
    };

    let result = ActionInfo {
        id: user.id,
        username: user.username,
        firstname: user.firstname,
        lastname: user.lastname,
        remaining_time: token_response.remaining_time,
    };

    Ok(result)
}
