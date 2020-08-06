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

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use askama::Template;

mod models;
mod schema;
mod database;
mod templates;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[derive(Template)]
#[template(path = "all_users.html")]
struct AllUsers
{
    users: Vec<models::User>
}

async fn greeting(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");

    let all_users = web::block(move || database::find_all_users(&conn))
    .await
    .map_err(|e|
        {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(users) = all_users
    {
        // for user in users
        // {
        //     println!("{:?}", user.email);
        //
        // }
        let all_users = AllUsers { users }.render().unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(all_users))

    }
    else
    {
        Ok(HttpResponse::Ok().body("Hello actix---web again!"))
    }


    // println!("{:?}", all_users);



}


async fn register_user(pool: web::Data<DbPool>, user_data: web::Json<models::UserRegisterData>)
    -> Result<Result<String, database::MyError> , Error>
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


async fn sign_in_user(pool: web::Data<DbPool>, user_data: web::Json<models::UserSignInDataRequest>)
    -> Result<Result<HttpResponse, database::MyError>, Error>
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
        dotenv::dotenv().ok();
        let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let key = secret_key.as_bytes();

        let expiration_date = Utc::now() + Duration::minutes(1);
        let claims = models::Claims
            { user_name: user.user_name.to_string(), email: user.email.to_string(), exp: expiration_date.timestamp() as usize };
        let token = encode(&Header::default(), &claims,&EncodingKey::from_secret(key)).unwrap();

        Ok(Ok(HttpResponse::Ok().json(models::UserSignInDataResponse
            {
                // access_type: "XXX-XXX-XXX".to_string(),
                access_token: token.to_string()
            })))
    }
    else
    {
        Ok(Err(database::MyError::Unauthorized { message: "Incorrect user name or password.".to_string() } ))
    }
}


async fn identify_user(pool: web::Data<DbPool>, request: HttpRequest)
    -> Result<Result<HttpResponse, database::MyError>, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");

    dotenv::dotenv().ok();
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let key = secret_key.as_bytes();

    if let Some(received_token) = request.headers().get("authorization")
    {
        if let Ok(user_data) = decode::<models::Claims>(
            received_token.to_str().unwrap(), &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),)
        {
            let verified_user = web::block(move || database::verify_user_by_name_and_email(&user_data, &conn))
            .await
            .map_err(|e|
                {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            if let Some(user) = verified_user
            {
                Ok(Ok(HttpResponse::Ok().json(models::AuthorizedUserResponse
                    {
                        user_name: user.user_name,
                    })))
            }
            else
            {
                Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() } ))
            }
        }
        else
        {
            Ok(Err(database::MyError::Unauthorized { message: "Session has expired, please login again.".to_string() } ))
        }
    }
    else
    {
        Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() } ))
    }
}


async fn show_user_info(pool: web::Data<DbPool>, request: HttpRequest) -> Result<HttpResponse, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");

    dotenv::dotenv().ok();
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let key = secret_key.as_bytes();

    if let Some(received_token) = request.headers().get("authorization")
    {
        if let Ok(user_data) = decode::<models::Claims>(
            received_token.to_str().unwrap(), &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),)
        {
            let verified_user = web::block(move || database::verify_user_by_name_and_email(&user_data, &conn))
            .await
            .map_err(|e|
                {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            if let Some(user) = verified_user
            {
                let user_info = templates::AuthorizedUserInfo { user_name: &user.user_name, email: &user.email }.render().unwrap();
                Ok(HttpResponse::Ok().content_type("text/html").body(user_info))
            }
            else
            {
                let user_info = templates::AuthorizedUserInfo { user_name: "undefined", email: "undefined" }.render().unwrap();
                Ok(HttpResponse::Ok().content_type("text/html").body(user_info))
            }
        }
        else
        {
            let user_info = templates::AuthorizedUserInfo { user_name: "undefined", email: "undefined" }.render().unwrap();
            Ok(HttpResponse::Ok().content_type("text/html").body(user_info))
        }
    }
    else
    {
        let user_info = templates::AuthorizedUserInfo { user_name: "undefined", email: "undefined" }.render().unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(user_info))
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
                            .route(web::post().to(sign_in_user)), )
                        .route("/identify_user", web::get().to(identify_user))

                        .route("/user_info", web::get().to(show_user_info)))

                .route("/greeting", web::get().to(greeting))


                .service(Files::new("", "./web_layout").index_file("index.html"))
        })
    .bind(&bind)?
    .run()
    .await
}
