use serde::{Deserialize, Serialize};

use crate::schema::users;
//
// #[derive(Debug, Clone, Serialize, Queryable, Insertable)]
// pub struct User
// {
//     pub id: String,
//     pub name: String,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NewUser
// {
//     pub name: String,
// }

#[derive(Debug, Serialize, Queryable, Insertable)]
#[table_name="users"]
pub struct User
{
    pub id: String,
    pub user_name: String,
    pub email: String,
    pub password: String
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
    pub access_type: String,
    pub access_token: String
}
