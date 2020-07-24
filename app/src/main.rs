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

async fn greeting() -> impl Responder
{
    HttpResponse::Ok().body("Hello actix---web again!")
}


#[derive(Fail, Debug)]
enum MyError {
    #[fail(display = "{}", message)]
    Unauthorized { message: String },
}


impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::Unauthorized {..} => StatusCode::UNAUTHORIZED,
        }
    }
}


async fn register_user(pool: web::Data<DbPool>, user_data: web::Json<models::UserRegisterData>) -> Result<Result<String, MyError> , Error>
{
    let greeting = "Registration was successfully completed!".to_string();

    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || insert_new_user(&user_data, &conn))
        .await
        .map_err(|e|
            {
                eprintln!("{}", e);

                HttpResponse::InternalServerError().finish()
            })?;
    match user
    {
        Some(_) => Ok(Ok(greeting)),
        None => Ok(Err(MyError::Unauthorized { message: "The user name or email are already in use.".to_string() }))
    }
    // Ok(greeting)

    //     let user = web::block(move || actions::insert_new_user(&form.name, &conn))


    // let bad_names = vec!["aaa", "bbb", "ccc"];
    // let bad_emails = vec!["aaa@aaa.com", "bbb@bbb.com", "ccc@ccc.com"];
    // // let greeting = format!("Welcome {}, {}!", user_data.user_name, user_data.email);
    // let greeting = "Registration was successfully completed!".to_string();
    //
    // match bad_names.iter().find(|&&x| x == user_data.user_name)
    // {
    //     Some(_) => Err(MyError::Unauthorized { message: "The name is already in use.".to_string() } ),
    //     None =>
    //         {
    //             match bad_emails.iter().find(|&&x| x == user_data.email)
    //             {
    //                 Some(_) => Err(MyError::Unauthorized { message: "The email is already in use.".to_string() }),
    //                 None => Ok(greeting)
    //             }
    //         }
    // }
}




fn insert_new_user(user_data: &web::Json<models::UserRegisterData>, conn: &PgConnection) -> Result<Option<models::User>, diesel::result::Error>
{
    use crate::schema::users::dsl::*;

    match users
        .filter(user_name.eq(user_data.user_name.to_string()))
        .or_filter(email.eq(user_data.email.to_string()))
        .first::<models::User>(conn)
    {
        Ok(_) => Ok(None),
        Err(_) =>
            {
                let new_user = models::User
                {
                    id: Uuid::new_v4().to_string(),
                    user_name: user_data.user_name.to_owned(),
                    email: user_data.email.to_owned(),
                    password: user_data.password.to_owned()
                };
                match diesel::insert_into(users).values(&new_user).execute(conn)
                {
                    Ok(_) => Ok(Some(new_user)),
                    Err(e) => Err(e)
                }
            }
    }


    //
    // let new_user = models::User
    // {
    //     id: Uuid::new_v4().to_string(),
    //     user_name: user_data.user_name.to_owned(),
    //     email: user_data.email.to_owned(),
    //     password: user_data.password.to_owned()
    // };
    // diesel::insert_into(users).values(&new_user).execute(conn)?;
    // Ok(new_user)
}


async fn sign_in_user(user_data: web::Json<models::UserSignInDataRequest>) -> Result<HttpResponse, MyError>
{
    let good_names = vec!["ddd", "eee", "fff"];
    let good_passwords = vec!["ddd_pass", "eee_pass", "fff_pass"];
    let mut names_with_passwords: HashMap<_, _> =
        good_names.into_iter().zip(good_passwords.into_iter()).collect();

    match names_with_passwords.get(&user_data.user_name[..])
    {
        Some(password) => if password.to_string() == user_data.password
            {
                Ok(HttpResponse::Ok().json(models::UserSignInDataResponse
                    {
                        user_name: user_data.user_name.to_string(),
                        access_type: "XXX-XXX-XXX".to_string(),
                        access_token: "YYY-YYY-YYY".to_string()
                    }))
            }
            else
            {
                Err(MyError::Unauthorized { message: "Incorrect user name or password.".to_string() } )
            },
        None => Err(MyError::Unauthorized { message: "Incorrect user name or password.".to_string() } )
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

                .route("/greeting", web::get().to(greeting))
                .route("/greeting/greeting_2", web::get().to(greeting))
                .service(Files::new("", "./web_layout").index_file("index.html"))
                // .service(greeting)
        })
    .bind(&bind)?
    .run()
    .await
}
