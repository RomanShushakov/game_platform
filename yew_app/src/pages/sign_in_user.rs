use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use yew::format::Json;
use anyhow::Error;

use std::rc::Rc;

use crate::types::{AuthorizedUserResponse, UserSignInDataRequest, UserSignInDataResponse};


pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub user: Rc<Option<AuthorizedUserResponse>>,
    pub save_token: Callback<String>,
    pub identify_user: Callback<String>
}


struct State
{
    user_name: Option<String>,
    password: Option<String>,
    error_message: Option<String>
}


pub struct SignInUser
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    fetch_task: Option<FetchTask>,
}


pub enum Msg
{
    UpdateEditUserName(String),
    UpdateEditPassword(String),
    Login,
    SignInUser(UserSignInDataRequest),
    SuccessfulSignIn(Result<UserSignInDataResponse, Error>),
    UnsuccessfulSignIn
}


impl SignInUser
{
    fn sign_in_user(&self, sign_in_data: UserSignInDataRequest) -> FetchTask
    {
        let callback: FetchCallback<UserSignInDataResponse> = self.link.callback(
            move |response: FetchResponse<UserSignInDataResponse>|
                {
                    let (meta, Json(user_data)) = response.into_parts();
                    if meta.status.is_success()
                    {
                        Msg::SuccessfulSignIn(user_data)
                    }
                    else
                    {
                        Msg::UnsuccessfulSignIn
                    }
                },
            );

        let request = Request::post("/auth/sign_in_user")
            .header("Content-Type", "application/json")
            .body(Json(&sign_in_data))
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }


    fn check_input_fields(&self) -> Option<UserSignInDataRequest>
    {
        if let Some(user_name) = &self.state.user_name
        {
            if let Some(password) = &self.state.password
            {
                if !password.is_empty() && !user_name.is_empty()
                {
                    return Some(UserSignInDataRequest { user_name: user_name.to_owned(), password: password.to_owned() })
                }
            }
        }
        None
    }
}


impl Component for SignInUser
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
        {
            link, props, fetch_task: None,
            state: State { user_name: None, password: None, error_message: None }
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::UpdateEditUserName(e) => self.state.user_name = Some(e),
            Msg::UpdateEditPassword(e) => self.state.password = Some(e),
            Msg::Login =>
                {
                    if let Some(sign_in_data) = self.check_input_fields()
                    {
                        self.link.send_message(Msg::SignInUser(sign_in_data));
                    }
                    else
                    {
                        yew::services::dialog::DialogService::alert("Please fill all required fields.");
                    }
                },
            Msg::SignInUser(sign_in_data) =>
                {
                    let task = self.sign_in_user(sign_in_data);
                    self.fetch_task = Some(task);
                },
            Msg::SuccessfulSignIn(response) =>
                {
                    self.state.error_message = None;
                    let user_data = response.ok();
                    if let Some(user_data) = user_data
                    {
                        self.props.save_token.emit(user_data.access_token.to_owned());
                        self.props.identify_user.emit(user_data.access_token.to_owned());
                    }
                },
            Msg::UnsuccessfulSignIn =>
                {
                    self.state.error_message = Some("Incorrect user name or password.".to_owned());
                }
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
                        if let Some(user) = &*self.props.user
                        {
                            html! { <h3>{ format!("Hello, {}!", user.user_name) }</h3> }
                        }
                        else
                        {
                            html!
                            {
                                <>
                                    <h3>{ "Sign in" }</h3>
                                    <input
                                        class="authentication_input_field" placeholder="user name"
                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditUserName(e.value)) />
                                    <input
                                        class="authentication_input_field" type="password" placeholder="password"
                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditPassword(e.value)) />
                                    <button class="button" onclick=self.link.callback(|_| Msg::Login)>{ "Login" }</button>
                                    {
                                        if let Some(message) = &self.state.error_message
                                        {
                                            html! { <h4> { message } </h4> }
                                        }
                                        else
                                        {
                                            html! { }
                                        }
                                    }
                                </>
                            }
                        }
                    }
                </div>
            </main>
        }
    }
}
