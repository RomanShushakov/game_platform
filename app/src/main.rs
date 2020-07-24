#[macro_use]
extern crate diesel;

use actix_web::{error, web, FromRequest, HttpResponse, Responder, HttpServer, App, HttpRequest, Result, Resource};
use actix_http::ResponseBuilder;
use actix_web::{http::header, http::StatusCode};
use actix_files::Files;
use serde::{Serialize, Deserialize};
use failure::Fail;
use std::collections::HashMap;

use actix_web::{get, middleware, post, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use diesel::expression::exists::exists;

mod models;
mod schema;
mod database;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// async fn greeting() -> impl Responder
// {
//     HttpResponse::Ok().body("Hello actix---web again!")
// }


async fn register_user(pool: web::Data<DbPool>, user_data: web::Json<models::UserRegisterData>) -> Result<Result<String, database::MyError> , Error>
{
    let greeting = "Registration was successfully completed!".to_string();

    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || database::insert_new_user(&user_data, &conn))
        .await
        .map_err(|e|
            {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    match user
    {
        Ok(_)=> Ok(Ok(greeting)),
        Err(err) => Ok(Err(err))
    }
}


async fn sign_in_user(pool: web::Data<DbPool>, user_data: web::Json<models::UserSignInDataRequest>) -> Result<Result<HttpResponse, database::MyError>, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || database::find_user_by_name_and_password(&user_data, &conn))
        .await
        .map_err(|e|
            {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    if let Some(user) = user
        {
            Ok(Ok(HttpResponse::Ok().json(models::UserSignInDataResponse
                {
                    user_name: user.user_name.to_string(),
                    access_type: "XXX-XXX-XXX".to_string(),
                    access_token: "YYY-YYY-YYY".to_string()
                })))
        }
        else
        {
            Ok(Err(database::MyError::Unauthorized { message: "Incorrect user name or password.".to_string() } ))
        }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "0.0.0.0:8080";
    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move ||
        {
            App::new()
                // set up DB pool to be used with web::Data<Pool> extractor
                .data(pool.clone())
                .wrap(middleware::Logger::default())

                .service(
                    web::scope("/auth")
                        .service(
                        web::resource("/register_user")
                            // change json extractor configuration
                            .app_data(web::JsonConfig::default().limit(1024)
                                .error_handler(|err, _req|
                                    {
                                        // create custom error response
                                        error::InternalError::from_response(
                                            err,HttpResponse::Conflict().body("Incorrect data."), ).into()
                                    }
                                )
                            )
                            .route(web::post().to(register_user)), )
                        .service(web::resource("/sign_in_user")
                            // change json extractor configuration
                            .app_data(web::Json::<models::UserSignInDataRequest>::configure(
                                |cfg|
                                    {
                                        cfg.limit(1024).error_handler(|err, _req|
                                            {
                                                // create custom error response
                                                error::InternalError::from_response(
                                                    err,HttpResponse::Conflict().body("Incorrect data."), ).into()
                                            }
                                        )
                                    }
                                )
                            )
                            // .route(web::post().to(register_user)),)
                            .route(web::post().to(sign_in_user)), ))

                // .route("/greeting", web::get().to(greeting))
                // .route("/greeting/greeting_2", web::get().to(greeting))
                .service(Files::new("", "./web_layout").index_file("index.html"))
                // .service(greeting)
        })
    .bind(&bind)?
    .run()
    .await
}
