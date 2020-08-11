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


impl Default for User
{
    fn default() -> Self
    {
        Self
        {
            id: "undefined".to_string(),
            user_name: "undefined".to_string(),
            email: "undefined".to_string(),
            password: "undefined".to_string(),
            is_superuser: false,
            is_active: false
        }
    }
}


/// deserialize `UserRegisterData` from request's body, max payload size is 1kb
#[derive(Deserialize)]
pub struct UserRegisterData
{
    pub user_name: String,
    pub email: String,
    pub password: String
}

/// deserialize `UserSignInDataRequest` from request's body, max payload size is 1kb
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


#[derive(Serialize)]
pub struct AuthorizedUserResponse
{
    pub user_name: String,
}


#[derive(Deserialize)]
pub struct UserUpdateDataRequest
{
    pub edited_user_name: Option<String>,
    pub edited_email: Option<String>,
    pub edited_password: Option<String>
}
