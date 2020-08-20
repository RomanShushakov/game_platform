use serde::{Deserialize};


#[derive(Deserialize, Clone)]
pub struct User
{
    pub user_name: String
}
