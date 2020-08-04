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
    pub user_name: String,
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
    pub email: String
}
