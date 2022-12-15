use std::fs::File;
use std::io::Read;
use actix_web::{web, App, HttpServer};
use futures::io;
use crate::controller::user::{auth, get_user, list, user_create, who_ami};
use crate::entity::datastore::Datastore;
use crate::model::db::{Db, init_db};

mod model;
mod handle;
mod entity;
mod controller;

fn read_config(path: String) -> Result<Datastore, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let datastore: Datastore = serde_json::from_str(&s).unwrap();

    Ok(datastore)
}

#[actix_web::main]
async fn main() -> io::Result<()> {

    let pg_data = match read_config("config.json".to_string()) {
        Ok(res) => {
            res
        },
        Err(err) => {
            panic!("Error at read config.json: {}", err)
        }
    };

    let db = match init_db(&pg_data).await {
        Ok(database) => {
            database
        },
        Err(err) => {
            panic!("Error init_db(): {:?}", err)
        }
    };
    let db_data = web::Data::new(db);

    println!("<--START-SERVER--> 8081");
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/api/v1/user/{id}/get", web::get().to(get_user))
            .route("/api/v1/user/create", web::post().to(user_create))
            .route("/api/v1/user/list", web::get().to(list))
            .route("/api/v1/user/auth", web::post().to(auth))
            .route("/api/v1/user/who-ami", web::get().to(who_ami))
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
