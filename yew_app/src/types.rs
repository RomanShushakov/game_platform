use serde::{Deserialize, Serialize};


#[derive(Deserialize, Clone)]
pub struct AuthorizedUserResponse
{
    pub user_name: String
}


#[derive(Serialize, Clone)]
pub struct UserSignInDataRequest
{
    pub user_name: String,
    pub password: String
}


#[derive(Deserialize, Clone)]
pub struct UserSignInDataResponse
{
    pub access_token: String
}


#[derive(Serialize)]
pub struct UserRegisterData
{
    pub user_name: String,
    pub email: String,
    pub password: String
}