use super::db::Db;
use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct User {
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

pub struct TodoUser;

impl TodoUser {
    pub async fn create(db: &Db, request: UserRequest) -> Result<(), sqlx::Error> {
        let sql = "INSERT INTO tbl_user(username, password, firstname, lastname) VALUES($1, $2, $3, $4) RETURNING id, username, firstname, lastname";
        let query = sqlx::query_as::<_, User>(&sql)
            .bind(request.username)
            .bind(request.password)
            .bind(request.firstname)
            .bind(request.lastname);

        let _user = query.fetch_one(db).await?;

        Ok(())
    }

    pub async fn list(db: &Db) -> Result<Vec<User>, sqlx::Error> {
        let sql = "SELECT id, username, firstname, lastname FROM tbl_user ORDER BY id";
        let query = sqlx::query_as(&sql);
        let users = query.fetch_all(db).await?;

        Ok(users)
    }

    pub async fn get(id: i32, db: &Db) -> Result<User, sqlx::Error> {
        let sql = "SELECT id, username, firstname, lastname FROM tbl_user WHERE id=$1";
        let query = sqlx::query_as::<_, User>(&sql).bind(id);
        let user = query.fetch_one(db).await?;

        Ok(user)
    }

    pub async fn get_by_username(username: String, db: &Db) -> Result<UserGetByUsername, sqlx::Error> {
        let sql = "SELECT id, username, password, firstname, lastname FROM tbl_user WHERE username=$1";
        let query = sqlx::query_as::<_, UserGetByUsername>(&sql).bind(username);
        let ans = query.fetch_one(db).await?;

        Ok(ans)
    }
}
