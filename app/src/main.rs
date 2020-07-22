use actix_web::{error, web, FromRequest, HttpResponse, Responder, HttpServer, App, HttpRequest, Result};
use actix_http::ResponseBuilder;
use actix_web::{http::header, http::StatusCode};
use listenfd::ListenFd;
use actix_files::Files;
use serde::{Serialize, Deserialize};
use failure::Fail;
use std::collections::HashMap;


async fn greeting() -> impl Responder
{
    HttpResponse::Ok().body("Hello actix---web again!")
}

/// deserialize `UserData` from request's body, max payload size is 1kb
#[derive(Deserialize)]
struct UserRegisterData {
    user_name: String,
    email: String,
    password: String
}

#[derive(Deserialize)]
struct UserSignInDataRequest {
    user_name: String,
    password: String
}

#[derive(Serialize)]
struct UserSignInDataResponse {
    user_name: String,
    access_type: String,
    access_token: String
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

async fn register_user(user_data: web::Json<UserRegisterData>) -> Result<String, MyError>
{
    let bad_names = vec!["aaa", "bbb", "ccc"];
    let bad_emails = vec!["aaa@aaa.com", "bbb@bbb.com", "ccc@ccc.com"];
    // let greeting = format!("Welcome {}, {}!", user_data.user_name, user_data.email);
    let greeting = "Registration was successfully completed!".to_string();

    match bad_names.iter().find(|&&x| x == user_data.user_name)
    {
        Some(_) => Err(MyError::Unauthorized { message: "The name is already in use.".to_string() } ),
        None =>
            {
                match bad_emails.iter().find(|&&x| x == user_data.email)
                {
                    Some(_) => Err(MyError::Unauthorized { message: "The email is already in use.".to_string() }),
                    None => Ok(greeting)
                }
            }
    }
}

async fn sign_in_user(user_data: web::Json<UserSignInDataRequest>) -> Result<HttpResponse>
{
    let good_names = vec!["ddd", "eee", "fff"];
    let good_passwords = vec!["ddd_pass", "eee_pass", "fff_pass"];
    let mut names_with_passwords: HashMap<_, _> =
        good_names.into_iter().zip(good_passwords.into_iter()).collect();

    match names_with_passwords.get(&user_data.user_name[..])
    {
        Some(pass) => Ok(HttpResponse::Ok().json(UserSignInDataResponse {
                                user_name: user_data.user_name.to_string(), access_type: "XXX-XXX-XXX".to_string(), access_token: "YYY-YYY-YYY".to_string()
                            })),
        None => Ok(HttpResponse::Ok().json(UserSignInDataResponse {
                user_name: "ERR".to_string(), access_type: "ERR".to_string(), access_token: "ERR".to_string()
    }))

    }

}


#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(||
        {
            App::new()
                .service(
                    web::scope("/auth")
                        .service(
                        web::resource("/register_user")
                            // change json extractor configuration
                            .app_data(web::Json::<UserRegisterData>::configure(
                                |cfg|
                                    {
                                        cfg.limit(1024).error_handler(|err, _req|
                                            {
                                                // create custom error response
                                                error::InternalError::from_response(
                                                    err,HttpResponse::Conflict().finish(), ).into()
                                            }
                                        )
                                    }
                                )
                            )
                            .route(web::post().to(register_user)), )
                        .service(web::resource("/sign_in_user")
                            // change json extractor configuration
                            .app_data(web::Json::<UserSignInDataRequest>::configure(
                                |cfg|
                                    {
                                        cfg.limit(1024).error_handler(|err, _req|
                                            {
                                                // create custom error response
                                                error::InternalError::from_response(
                                                    err, HttpResponse::Conflict().finish(), ).into()
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
        });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap()
    {
        server.listen(l)?
    }
    else
    {
        // server.bind("127.0.0.1:8080")?
        server.bind("0.0.0.0:8080")?
    };
    server.run().await
}
