use serde::{Deserialize, Serialize};


#[derive(Deserialize, PartialEq, Clone)]
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


#[derive(Serialize, Debug)]
pub struct WsRequest
{
    pub action: String,
    pub data: String,
}


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WsResponse
{
    pub action: String,
    pub data: String,
}


#[derive(Deserialize)]
pub struct ChatMessageResponse
{
    pub user_name: String,
    pub message: String
}


#[derive(PartialEq, Clone)]
pub struct ChatMessage(pub String);


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct OnlineUser(pub String);


#[derive(PartialEq, Clone)]
pub struct SentInvitation
{
    pub to_user: String
}


pub struct ReceivedInvitation
{
    pub from_user: String
}
