use serde::{Deserialize, Serialize};

use crate::schema::users_data;


#[derive(Debug, Serialize, Queryable, Insertable)]
#[table_name="users_data"]
pub struct User
{
    pub id: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub is_superuser: bool,
    pub is_active: bool
}


#[derive(Deserialize)]
pub struct UserRegisterData
{
    pub user_name: String,
    pub email: String,
    pub password: String
}


#[derive(Deserialize)]
pub struct UserSignInDataRequest
{
    pub user_name: String,
    pub password: String
}


#[derive(Serialize)]
pub struct UserSignInDataResponse
{
    // pub access_type: String,
    pub access_token: String
}


#[derive(Serialize, Deserialize)]
pub struct Claims
    {
        pub user_name: String,
        pub email: String,
        pub exp: usize,
    }


// #[derive(Serialize)]
// pub struct AuthorizedUserResponse
// {
//     pub user_name: String,
// }


#[derive(Serialize)]
pub struct AuthorizedUserResponse
{
    pub user_name: String,
    pub email: String,
    pub is_superuser: bool
}


#[derive(Deserialize)]
pub struct UserUpdateDataRequest
{
    pub edited_user_name: Option<String>,
    pub edited_email: Option<String>,
    pub edited_password: Option<String>
}


#[derive(Serialize)]
pub struct AuthorizedUserInfo
{
    pub user_name: String,
    pub email: String
}


#[derive(Serialize)]
pub struct AuthorizedSuperUserInfo
{
    pub user_name: String,
    pub email: String
}


#[derive(Serialize)]
pub struct UserForAllUsersResponse
{
    pub id: String,
    pub user_name: String,
    pub email: String,
    pub is_active: bool
}


impl Default for UserForAllUsersResponse
{
    fn default() -> Self
    {
        Self
        {
            id: "undefined".to_string(),
            user_name: "undefined".to_string(),
            email: "undefined".to_string(),
            is_active: false
        }
    }
}


#[derive(Deserialize)]
pub struct UserStatusChangeRequest
{
    pub uid: String
}
