use askama::Template;


#[derive(Template)]
#[template(path = "user_info.html")]
pub struct AuthorizedUserInfo<'a>
{
    pub user_name: &'a str,
    pub email: &'a str
}