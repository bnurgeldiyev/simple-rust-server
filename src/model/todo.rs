use super::db::Db;
use crate::entity::user::{UserBase, UserGetByUsername, UserRequest};

pub struct TodoUser;

impl TodoUser {
    pub async fn create(db: &Db, request: UserRequest) -> Result<(), sqlx::Error> {
        let sql = "INSERT INTO tbl_user(username, password, firstname, lastname) VALUES($1, $2, $3, $4) RETURNING id, username, firstname, lastname";
        let query = sqlx::query_as::<_, UserBase>(&sql)
            .bind(request.username)
            .bind(request.password)
            .bind(request.firstname)
            .bind(request.lastname);

        let _user = query.fetch_one(db).await?;

        Ok(())
    }

    pub async fn list(db: &Db) -> Result<Vec<UserBase>, sqlx::Error> {
        let sql = "SELECT id, username, firstname, lastname FROM tbl_user ORDER BY id";
        let query = sqlx::query_as(&sql);
        let users = query.fetch_all(db).await?;

        Ok(users)
    }

    pub async fn get(id: i32, db: &Db) -> Result<UserBase, sqlx::Error> {
        let sql = "SELECT id, username, firstname, lastname FROM tbl_user WHERE id=$1";
        let query = sqlx::query_as::<_, UserBase>(&sql).bind(id);
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
