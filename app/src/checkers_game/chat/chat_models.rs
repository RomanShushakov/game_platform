use serde::{Deserialize, Serialize};

use crate::schema::checkers_game_chat;


#[derive(Deserialize, Debug)]
pub struct WsRequest
{
    pub action: String,
    pub text: String,
}


#[derive(Serialize, Debug)]
pub struct WsResponse
{
    pub text: String,
}


#[derive(Insertable)]
#[table_name="checkers_game_chat"]
pub struct ChatMessage
{
    pub chat_room: String,
    pub user_name: String,
    pub message: String
}


#[derive(Debug, Serialize, Queryable)]
pub struct ChatMessageResponse
{
    pub id: i32,
    pub chat_room: String,
    pub user_name: String,
    pub message: String
}


#[derive(Deserialize)]
pub struct Info
{
    pub room: String,
}
