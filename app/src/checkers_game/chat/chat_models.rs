use serde::{Deserialize, Serialize};


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
