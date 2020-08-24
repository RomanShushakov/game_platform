#[macro_use]
extern crate diesel;

use actix_web::{error, web, FromRequest, HttpResponse, HttpServer, App, HttpRequest, Result};
use actix_files::Files;

use actix_web::{middleware, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};


use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, TokenData};

use askama::Template;
use crate::database::MyError;

mod models;
mod schema;
mod database;
mod templates;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn register_user(pool: web::Data<DbPool>, user_data: web::Json<models::UserRegisterData>)
    -> Result<Result<HttpResponse, database::MyError> , Error>
{
    let message = "Registration was successfully completed!".to_string();

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
        Ok(_)=> Ok(Ok(HttpResponse::Ok().body(message))),
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

        let expiration_date = Utc::now() + Duration::minutes(360);
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


async fn decode_token(token: &str) -> Result<TokenData<models::Claims>, database::MyError>
{
    dotenv::dotenv().ok();
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let key = secret_key.as_bytes();
    let decoded_token = decode::<models::Claims>(
        token, &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS256),);
    match decoded_token
    {
        Ok(token) => Ok(token),
        Err(_) => Err(database::MyError::Unauthorized { message: "Session has expired, please login again.".to_string() } )
    }
}


async fn verify_user(user_data: TokenData<models::Claims>, pool: &web::Data<DbPool>)
    -> Result<Option<models::User>, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");
    let verified_user = web::block(move || database::verify_user_by_name_and_email(&user_data, &conn))
    .await
    .map_err(|e|
        {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(verified_user)
}


async fn identify_user(pool: web::Data<DbPool>, request: HttpRequest)
    -> Result<Result<HttpResponse, database::MyError>, Error>
{
    if let Some(received_token) = request.headers().get("authorization")
    {
        match decode_token(received_token.to_str().unwrap()).await
        {
            Ok(user_data) =>
                {
                    let verified_user = verify_user(user_data, &pool).await;
                    match verified_user
                    {

                        // Ok(Some(user)) => Ok(Ok(HttpResponse::Ok().json(models::AuthorizedUserResponse { user_name: user.user_name } )) ),

                        Ok(Some(user)) => Ok(Ok(HttpResponse::Ok().json(models::AuthorizedUserResponse
                            {
                                user_name: user.user_name,
                                email: user.email, is_superuser: user.is_superuser
                            }))
                        ),

                        Ok(None) => Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() })),
                        Err(e) => Err(e)
                    }
                },
            Err(e) => Ok(Err(e))
        }
    }
    else
    {
        Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() } ))
    }
}


async fn show_user_info(pool: web::Data<DbPool>, request: HttpRequest) -> Result<HttpResponse, Error>
{
    let user_info = templates::AuthorizedUserInfo { user_name: "undefined", email: "undefined" }.render().unwrap();
    let undefined_user_response = Ok(HttpResponse::Ok().content_type("text/html").body(user_info));

    if let Some(received_token) = request.headers().get("authorization")
    {
        if let Ok(decoded_user) = decode_token(received_token.to_str().unwrap()).await
        {
            let verified_user = verify_user(decoded_user, &pool).await;
            match verified_user
            {
                Ok(Some(user)) =>
                    {
                        if user.is_superuser
                        {
                            let user_info = templates::AuthorizedSuperUserInfo { user_name: &user.user_name, email: &user.email }.render().unwrap();
                            Ok(HttpResponse::Ok().content_type("text/html").body(user_info))
                        }
                        else
                        {
                            let user_info = templates::AuthorizedUserInfo { user_name: &user.user_name, email: &user.email }.render().unwrap();
                            Ok(HttpResponse::Ok().content_type("text/html").body(user_info))
                        }
                    },
                Ok(None) => undefined_user_response,
                Err(e) => Err(e)
            }
        }
        else { undefined_user_response }
    }
    else { undefined_user_response }
}


async fn update_user_data(pool: &web::Data<DbPool>, edited_user_data: web::Json<models::UserUpdateDataRequest>, uid: String)
    -> Result<Result<(), MyError>, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");
    let updating_process = web::block(move || database::update_user_data(&edited_user_data, &conn, &uid))
        .await
        .map_err(|e|
            {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    Ok(updating_process)
}


async fn update_user(pool: web::Data<DbPool>, edited_user_data: web::Json<models::UserUpdateDataRequest>, request: HttpRequest)
    -> Result<Result<HttpResponse, database::MyError> , Error>
{
    let message = "User data were successfully updated!".to_string();

    if let Some(received_token) = request.headers().get("authorization")
    {
        match decode_token(received_token.to_str().unwrap()).await
        {
            Ok(user_data) =>
                {
                    let verified_user = verify_user(user_data, &pool).await;
                    match verified_user
                    {
                        Ok(Some(user)) =>
                            {
                                let updated_user = update_user_data(&pool, edited_user_data, user.id).await;
                                match updated_user
                                {
                                    Ok(Ok(_)) => Ok(Ok(HttpResponse::Ok().body(message))),
                                    Ok(Err(e)) => Ok(Err(e)),
                                    Err(e) => Err(e)
                                }
                            },
                        Ok(None) => Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() } )),
                        Err(e) => Err(e)
                    }

                },
            Err(e) => Ok(Err(e))
        }
    }
    else
    {
        Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() } ))
    }
}


async fn extract_users_data(pool: &web::Data<DbPool>) -> Result<Option<Vec<models::User>>, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");
    let all_users = web::block(move || database::extract_users_data(&conn))
    .await
    .map_err(|e|
        {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(all_users)
}


async fn show_users(pool: web::Data<DbPool>, request: HttpRequest) -> Result<HttpResponse, Error>
{
    let undefined_user = models::User::default();
    let users = vec![undefined_user];
    let undefined_users = templates::AllUsers { users }.render().unwrap();
    let undefined_users_response = Ok(HttpResponse::Ok().content_type("text/html").body(undefined_users));

    if let Some(received_token) = request.headers().get("authorization")
    {
        if let Ok(decoded_user) = decode_token(received_token.to_str().unwrap()).await
        {
            if let Ok(Some(verified_user)) = verify_user(decoded_user, &pool).await
            {
                if verified_user.is_superuser
                {
                    let all_users = extract_users_data(&pool).await;
                    match all_users
                    {
                        Ok(Some(users)) =>
                            {
                                let all_users = templates::AllUsers { users }.render().unwrap();
                                Ok(HttpResponse::Ok().content_type("text/html").body(all_users))
                            },
                        Ok(None) => undefined_users_response,
                        Err(e) => Err(e)
                    }
                }
                else { undefined_users_response }
            }
            else { undefined_users_response }
        }
        else { undefined_users_response }
    }
    else { undefined_users_response }
}


async fn change_user_activity(pool: &web::Data<DbPool>, uid: String) -> Result<Result<(), database::MyError>, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");
    let updated_user = web::block(move || database::change_user_activity(&conn, &uid))
        .await
        .map_err(|e|
            {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    Ok(updated_user)
}


async fn change_user_status(user: web::Json<models::UserStatusChangeRequest>, pool: web::Data<DbPool>, request: HttpRequest)
    -> Result<Result<HttpResponse, database::MyError>, Error>
{
    if let Some(received_token) = request.headers().get("authorization")
    {
        match decode_token(received_token.to_str().unwrap()).await
        {
            Ok(user_data) =>
                {
                    let verified_user = verify_user(user_data, &pool).await;
                    match verified_user
                    {
                        Ok(Some(verified_user)) =>
                            {
                                if verified_user.is_superuser
                                {
                                    let updated_user = change_user_activity(&pool, user.uid.to_string()).await;
                                    match updated_user
                                    {
                                        Ok(Ok(_)) => Ok(Ok(HttpResponse::Ok().content_type("text/html").body("User status was successfully changed."))),
                                        Ok(Err(e)) => Ok(Err(e)),
                                        Err(e) => Err(e)
                                    }
                                }
                                else
                                {
                                    Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() } ))
                                }
                            },
                        Ok(None) => Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() } )),
                        Err(e) => Err(e)
                    }
                },
            Err(e) => Ok(Err(e))
        }
    }
    else
    {
        Ok(Err(database::MyError::Unauthorized { message: "Something go wrong.".to_string() } ))
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

                        .route("/user_info", web::get().to(show_user_info))

                        .service(web::resource("/update_user")
                            // change json extractor configuration
                            .app_data(web::Json::<models::UserUpdateDataRequest>::configure(
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
                            .route(web::post().to(update_user)), )

                        .route("/all_users", web::get().to(show_users))

                        .route("/change_user_status", web::post().to(change_user_status)))

                // .service(Files::new("", "./web_layout").index_file("index.html"))
                .service(Files::new("", "../yew_app/web_layout").index_file("index.html"))
        })
    .bind(&bind)?
    .run()
    .await
}
