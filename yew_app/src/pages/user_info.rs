use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::Json;
use anyhow::Error;

use validator;

use crate::types::{AuthorizedUserResponse, UserUpdateDataRequest};
use crate::KEY;

use crate::components::AllUsers;


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
    pub token: Option<String>,
    pub sign_out: Callback<()>,
}


struct State
{
    edited_user_name: Option<String>,
    edited_email: Option<String>,
    edited_password: Option<String>,
    edited_retyped_password: Option<String>,
    error_message: Option<String>,
    data_update_success_message: Option<String>
}


impl Default for State
{
    fn default() -> Self
        {
            Self
            {
                edited_user_name: None,
                edited_email: None,
                edited_password: None,
                edited_retyped_password: None,
                error_message: None,
                data_update_success_message: None,
            }
        }
}


pub struct UserInfo
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    fetch_task: Option<FetchTask>,
}


pub enum Msg
{
    UpdateEditUserName(String),
    UpdateEditEmail(String),
    UpdateEditPassword(String),
    UpdateEditRetypePassword(String),
    Save,
    UpdateUser(UserUpdateDataRequest),
    SuccessfulUpdate(Result<String, Error>),
    UnsuccessfulUpdate(Result<String, Error>)
}


impl UserInfo
{
    fn compose_updated_user_data(&self) -> Option<UserUpdateDataRequest>
    {
        let updated_user_name: Option<String> = match &self.state.edited_user_name
        {
            Some(user_name) =>
                {
                    if !user_name.is_empty()
                    {
                        Some(user_name.to_string())
                    }
                    else { None }
                }
            None => None
        };

        let updated_email: Option<String> = match &self.state.edited_email
        {
            Some(email) =>
                {
                    if !email.is_empty()
                    {
                        Some(email.to_string())
                    }
                    else { None }
                }
            None => None
        };

        let updated_password: Option<String> = match &self.state.edited_password
        {
            Some(password) =>
                {
                    if !password.is_empty()
                    {
                        Some(password.to_string())
                    }
                    else { None }
                }
            None => None
        };

        let updated_retyped_password: Option<String> = match &self.state.edited_retyped_password
        {
            Some(password) =>
                {
                    if !password.is_empty()
                    {
                        Some(password.to_string())
                    }
                    else { None }
                }
            None => None
        };

        if let Some(email) = &updated_email
        {
            if !validator::validate_email(email)
            {
                yew::services::dialog::DialogService::alert("You have entered an invalid email address.");
                return None
            }
        }

        if let Some(password) = &updated_password
        {
            if let Some(retyped_password) = &updated_retyped_password
            {
                if password != retyped_password
                {
                    yew::services::dialog::DialogService::alert("Password doesn't match!");
                    return None
                }
            }
            else
            {
                yew::services::dialog::DialogService::alert("Password doesn't match!");
                return None
            }
        }

        if let Some(_) = &updated_retyped_password
        {
            if let None = &updated_password
            {
                yew::services::dialog::DialogService::alert("Password doesn't match!");
                return None
            }
        }

        if updated_user_name.is_some() || updated_email.is_some() || updated_password.is_some()
        {
            Some(UserUpdateDataRequest
            {
                edited_user_name: updated_user_name, edited_email: updated_email, edited_password: updated_password
            })
        }
        else { None }
    }


    fn update_user(&self, updated_user_data: UserUpdateDataRequest, token: &str) -> FetchTask
    {
        let callback = self.link.callback(
            move |response: Response<Result<String, Error>>|
                {
                    let (meta, message) = response.into_parts();
                    if meta.status.is_success()
                    {
                        Msg::SuccessfulUpdate(message)
                    }
                    else
                    {
                        Msg::UnsuccessfulUpdate(message)
                    }
                },
            );

        let request = Request::post("/auth/update_user")
            .header("Content-Type", "application/json" )
            .header(KEY, token )
            .body(Json(&updated_user_data))
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}


impl Component for UserInfo
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
        {
            props, link, state: State::default(),
            fetch_task: None
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::UpdateEditUserName(e) =>
                {
                    self.state.edited_user_name = Some(e);
                },
            Msg::UpdateEditEmail(e) =>
                {
                    self.state.edited_email = Some(e);
                },
            Msg::UpdateEditPassword(e) =>
                {
                    self.state.edited_password = Some(e);
                },
            Msg::UpdateEditRetypePassword(e) => self.state.edited_retyped_password = Some(e),
            Msg::Save =>
                {
                    if let Some(updated_user_data) = self.compose_updated_user_data()
                    {
                        self.link.send_message(Msg::UpdateUser(updated_user_data));
                    }
                    else { return false }
                },
            Msg::UpdateUser(updated_user_data) =>
                {
                    if let Some(token) = &self.props.token
                    {
                        let task = self.update_user(updated_user_data, token);
                        self.fetch_task = Some(task);
                    }
                    else { return false }
                },
            Msg::SuccessfulUpdate(message) =>
                {
                    self.state.data_update_success_message = Some(message.unwrap());
                    self.props.sign_out.emit(());
                },
            Msg::UnsuccessfulUpdate(message) => self.state.error_message = Some(message.unwrap()),
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            true
        }
        else
        {
            false
        }
    }


    fn view(&self) -> Html
    {
        html!
        {
            <main class="main">
                <div class="container">
                    {
                        if let Some(success_message) = &self.state.data_update_success_message
                        {
                            html! { <h4>{ success_message }</h4> }
                        }
                        else
                        {
                            {
                                if let Some(user) = &self.props.user
                                {
                                    html!
                                    {
                                        <>
                                            <div class="user_info_container">
                                                <h3>{ "Edit profile." } </h3>

                                                <div>
                                                    <p>{ "User name:" }</p>
                                                    <input
                                                        placeholder={ &user.user_name }
                                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditUserName(e.value)) />
                                                </div>

                                                <div>
                                                    <p>{ "Email:" }</p>
                                                    <input
                                                        placeholder={ &user.email }
                                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditEmail(e.value)) />
                                                </div>

                                                <div>
                                                    <p>{ "Password:" }</p>
                                                    <input
                                                        type="password" placeholder="enter new password"
                                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditPassword(e.value)) />
                                                    <input
                                                        type="password" placeholder="retype new password"
                                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditRetypePassword(e.value)) />
                                                </div>

                                                <div class="apply_cancel_container">
                                                    <button class="button" onclick=self.link.callback(|_| Msg::Save)>{ "Save" }</button>
                                                </div>

                                                {
                                                    if let Some(error_message) = &self.state.error_message
                                                    {
                                                        html! { <h4>{ error_message }</h4> }
                                                    }
                                                    else
                                                    {
                                                        html! {  }
                                                    }
                                                }

                                            </div>

                                            {
                                                if user.is_superuser
                                                {
                                                    html! { <AllUsers token=self.props.token.clone() /> }
                                                }
                                                else
                                                {
                                                    html! {  }
                                                }
                                            }
                                        </>
                                    }
                                }
                                else
                                {
                                    html! { <h3>{ "Undefined user." } </h3> }
                                }
                            }
                        }
                    }
                </div>
            </main>
        }
    }
}
