use diesel::prelude::*;
use uuid::Uuid;
use actix_web::{web, error, HttpResponse};
use actix_http::ResponseBuilder;
use actix_web::{http::header, http::StatusCode};
use failure::Fail;

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use jsonwebtoken::TokenData;

use crate::models;


#[derive(Fail, Debug)]
pub enum MyError {
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


fn modify_password(password: &str) -> String
{
    let mut updated_password = String::new();
    for (i, char) in password.chars().rev().enumerate()
    {
        if i % 2 == 0
        {
            updated_password += &char.to_uppercase().to_string();
        }
        else
        {
            updated_password += &char.to_string();
        }
    }
    let mut modified_password = Sha256::new();
    modified_password.input_str(updated_password.as_str());
    modified_password.result_str()
}


pub fn insert_new_user(user_data: &web::Json<models::UserRegisterData>, conn: &PgConnection)
    -> Result<Result<models::User, MyError>, diesel::result::Error>
{
    use crate::schema::users_data::dsl::*;

    match users_data
        .filter(user_name.eq(user_data.user_name.to_string()))
        .or_filter(email.eq(user_data.email.to_string()))
        .first::<models::User>(conn)
    {
        Ok(existed_user) => if existed_user.user_name == user_data.user_name
            {
                Ok(Err(MyError::Unauthorized { message: "The name is already in use.".to_string() }))
            }
            else
            {
                Ok(Err(MyError::Unauthorized { message: "The email is already in use.".to_string() }))
            },
        Err(_) =>
            {
                let new_user = models::User
                {
                    id: Uuid::new_v4().to_string(),
                    user_name: user_data.user_name.to_owned(),
                    email: user_data.email.to_owned(),
                    password: modify_password(&user_data.password),
                    is_superuser: false,
                    is_active: true
                };
                match diesel::insert_into(users_data).values(&new_user).execute(conn)
                {
                    Ok(_) => Ok(Ok(new_user)),
                    Err(e) => Err(e)
                }
            }
    }
}


pub fn find_user_by_name_and_password(user_data: &web::Json<models::UserSignInDataRequest>, conn: &PgConnection)
    -> Result<Option<models::User>, diesel::result::Error>
{
    use crate::schema::users_data::dsl::*;

    let existed_user = users_data
        .filter(user_name.eq(user_data.user_name.to_string()))
        .filter(password.eq(modify_password(&user_data.password)))
        .first::<models::User>(conn)
        .optional()?;
    Ok(existed_user)
}


pub fn verify_user_by_name_and_email(user_data: &TokenData<models::Claims>, conn: &PgConnection)
    -> Result<bool, diesel::result::Error>
{
    use crate::schema::users_data::dsl::*;

    match users_data
        .filter(user_name.eq(user_data.claims.user_name.to_string()))
        .first::<models::User>(conn)
    {
        Ok(existed_user) => Ok(existed_user.email == user_data.claims.email.to_string()),
        Err(e) => Err(e)
    }
}
