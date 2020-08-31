use yew_router::prelude::*;


#[derive(Switch, Debug, Clone)]
pub enum AppRoute
{
    #[to = "/#auth/sign_in_user"]
    SignInUser,
    #[to = "/#auth/register_user"]
    RegisterUser,
    #[to = "/#auth/user_info"]
    UserInfo,
    #[to = "/#checkers"]
    CheckersGame,
    #[to = "/"]
    HomePage,
}
