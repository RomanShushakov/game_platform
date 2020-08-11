use crate::models;
use askama::Template;


#[derive(Template)]
#[template(path = "default_user_info.html")]
pub struct AuthorizedUserInfo<'a>
{
    pub user_name: &'a str,
    pub email: &'a str
}


#[derive(Template)]
#[template(path = "super_user_info.html")]
pub struct AuthorizedSuperUserInfo<'a>
{
    pub user_name: &'a str,
    pub email: &'a str
}


#[derive(Template)]
#[template(path = "all_users.html")]
pub struct AllUsers
{
    pub users: Vec<models::User>
}
