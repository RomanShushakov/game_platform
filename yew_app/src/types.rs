use serde::{Deserialize};


#[derive(Deserialize, Clone)]
pub struct AuthorizedUserResponse
{
    pub user_name: String
}
