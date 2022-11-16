use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::http::header::ContentType;
use crate::entity::user::UserBase;
use crate::handle::handle::{GeneralResponse, handle_user_auth, handle_user_create, handle_user_get, handle_user_list};
use crate::entity::token::{AccessTokenError, is_unauthorized };
use crate::model::db::{Db, init_db};

mod model;
mod handle;
mod entity;

async fn user_create(body: String, db: web::Data<Db>) -> impl Responder {
    let user = handle_user_create(&body, &db).await;
    web::Json(user)
}

async fn get_user(path: web::Path<i32>, db: web::Data<Db>) -> impl Responder {
    let id = path.into_inner();

    match handle_user_get(id, &db).await {
        Ok(res) => {
            HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&res).unwrap())
        },
        Err(err) => {
            let resp: GeneralResponse<UserBase> = GeneralResponse{
                status: false,
                data: None,
            };

            if format!("{:?}", sqlx::Error::RowNotFound) == format!("{:?}", err) {
                return HttpResponse::NotFound().content_type(ContentType::json()).body(serde_json::to_string(&resp).unwrap());
            }

            HttpResponse::InternalServerError().content_type(ContentType::json()).body(serde_json::to_string(&resp).unwrap())
        }
    }
}

async fn list(db: web::Data<Db>, req: HttpRequest) -> impl Responder {

    // verify token
    let _res = match is_unauthorized(&req, &db).await {
        Ok(res) => {
            res
        },
        Err(err) => {
            return if err == AccessTokenError::TokenInvalid {
                println!("Token invalid");

                let resp: GeneralResponse<UserBase> = GeneralResponse {
                    status: false,
                    data: None,
                };

                HttpResponse::Unauthorized().content_type(ContentType::json()).body(serde_json::to_string(&resp).unwrap())
            } else {
                println!("Token expired");
                let resp: GeneralResponse<UserBase> = GeneralResponse {
                    status: false,
                    data: None,
                };

                HttpResponse::Unauthorized().content_type(ContentType::json()).body(serde_json::to_string(&resp).unwrap())
            }
        },
    };

    // handle
    match handle_user_list(&db).await {
        Ok(res) => {
            HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&res).unwrap())
        },
        Err(_) => {

            let resp: GeneralResponse<UserBase> = GeneralResponse{
                status: false,
                data: None,
            };

            HttpResponse::InternalServerError().content_type(ContentType::json()).body(serde_json::to_string(&resp).unwrap())
        }
    }
}

async fn auth(body: String, db: web::Data<Db>) -> impl Responder {
    let res = handle_user_auth(&body, &db).await;

    web::Json(res)
}

async fn who_ami(req: HttpRequest, db: web::Data<Db>) -> impl Responder {

    match is_unauthorized(&req, &db).await {
        Ok(res) => {
            HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&res).unwrap())
        },
        Err(err) => {
            if err == AccessTokenError::TokenInvalid {
                println!("Token invalid");

                let resp: GeneralResponse<UserBase> = GeneralResponse {
                    status: false,
                    data: None,
                };

                HttpResponse::Unauthorized().content_type(ContentType::json()).body(serde_json::to_string(&resp).unwrap())
            } else {
                println!("Token expired");
                let resp: GeneralResponse<UserBase> = GeneralResponse {
                    status: false,
                    data: None,
                };

                HttpResponse::Unauthorized().content_type(ContentType::json()).body(serde_json::to_string(&resp).unwrap())
            }
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = match init_db().await {
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
        // .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
