use sqlx::{Pool, Postgres};
// use tokio::fs;
use sqlx::postgres::PgPoolOptions;
use crate::entity::datastore::Datastore;

pub type Db = Pool<Postgres>;

pub async fn init_db(data_store: &Datastore) -> Result<Db, sqlx::Error> {
    /*
    {
        let root_db = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await?;
        pexec(&root_db, SQL_RECREATE).await?;
    }*/

    new_db_pool(data_store.db_host.as_str(), data_store.db_name.as_str(), data_store.db_user.as_str(), data_store.db_password.as_str(), 5).await
}
/*
async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    let content = fs::read_to_string(file).await.map_err(|ex| {
        println!("ERROR reading {} (cause: {:?})", file, ex);
        ex
    })?;

    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(ex) => println!("WARNING - pexec - Sql file '{}' FAILED cause: {:?}", file, ex),
        }
    }

    Ok(())
}
*/
async fn new_db_pool(host: &str, db: &str, user: &str, pwd: &str, max_conn: u32) -> Result<Db, sqlx::Error> {
    let conn_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_conn)
        // .connect_timeout(Duration::from_millis(500))
        .connect(&conn_string)
        .await
}
