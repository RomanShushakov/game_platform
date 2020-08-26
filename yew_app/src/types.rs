use serde::{Deserialize, Serialize};


#[derive(Deserialize, Clone)]
pub struct AuthorizedUserResponse
{
    pub user_name: String,
    pub email: String,
    pub is_superuser: bool
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


#[derive(Serialize)]
pub struct UserUpdateDataRequest
{
    pub edited_user_name: Option<String>,
    pub edited_email: Option<String>,
    pub edited_password: Option<String>
}


#[derive(Deserialize)]
pub struct UserForAllUsersResponse
{
    pub id: String,
    pub user_name: String,
    pub email: String,
    pub is_active: bool
}


#[derive(Serialize, Clone)]
pub struct UserChangeStatusRequest
{
    pub uid: String,
}
