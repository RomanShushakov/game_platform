use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::route::AppRoute;
use yew_router::components::RouterAnchor;

use yew::format::Json;
use anyhow::Error;

use crate::types::UserRegisterData;

use validator;

// pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
// type FetchCallback<T> = Callback<FetchResponse<T>>;


struct State
{
    user_name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    registration_success_message: Option<String>,
    registration_error_message: Option<String>
}


pub struct RegisterUser
{
    link: ComponentLink<Self>,
    state: State,
    fetch_task: Option<FetchTask>,
}


pub enum Msg
{
    UpdateEditUserName(String),
    UpdateEditEmail(String),
    UpdateEditPassword(String),
    Submit,
    RegisterUser(UserRegisterData),
    SuccessfulRegistration(Result<String, Error>),
    UnsuccessfulRegistration(Result<String, Error>)
}

impl RegisterUser
{
    fn check_input_fields(&self) -> Option<UserRegisterData>
    {
        if let Some(user_name) = &self.state.user_name
        {
            if let Some(email) = &self.state.email
            {
                if let Some(password) = &self.state.password
                {
                    if !password.is_empty() && !email.is_empty() && !user_name.is_empty()
                    {
                        return Some(UserRegisterData {
                            user_name: user_name.to_owned(), email: email.to_owned(), password: password.to_owned()
                        })
                    }
                }
            }
        }
        None
    }


    fn register_user(&self, register_data: UserRegisterData) -> FetchTask
    {
        let callback = self.link.callback(
            move |response: Response<Result<String, Error>>|
                {
                    let (meta, message) = response.into_parts();
                    if meta.status.is_success()
                    {
                        Msg::SuccessfulRegistration(message)
                    }
                    else
                    {
                        Msg::UnsuccessfulRegistration(message)
                    }
                },
            );

        let request = Request::post("/auth/register_user")
            .header("Content-Type", "application/json")
            .body(Json(&register_data))
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}


impl Component for RegisterUser
{
    type Message = Msg;
    type Properties = ();


    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
        {
            link, fetch_task: None,
            state: State
                {
                    user_name: None, email: None, password: None,
                    registration_error_message: None, registration_success_message: None
                }
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::UpdateEditUserName(e) => self.state.user_name = Some(e),
            Msg::UpdateEditEmail(e) => self.state.email = Some(e),
            Msg::UpdateEditPassword(e) => self.state.password = Some(e),
            Msg::Submit =>
                {
                    if let Some(register_data) = self.check_input_fields()
                    {
                        if validator::validate_email(&register_data.email)
                        {
                            self.link.send_message(Msg::RegisterUser(register_data));
                        }
                        else
                        {
                            yew::services::dialog::DialogService::alert("You have entered an invalid email address.");
                        }
                    }
                    else
                    {
                        yew::services::dialog::DialogService::alert("Please fill all required fields.");
                    }
                },
            Msg::RegisterUser(register_data) =>
                {
                    let task = self.register_user(register_data);
                    self.fetch_task = Some(task);
                },
            Msg::SuccessfulRegistration(message) =>
                {
                    self.state.registration_success_message = Some(message.unwrap());
                }
            Msg::UnsuccessfulRegistration(message) =>
                {
                    self.state.registration_error_message = Some(message.unwrap());
                }
        }
        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }


    fn view(&self) -> Html
    {
        type Anchor = RouterAnchor<AppRoute>;

        html!
        {
            <main class="main">
                <div class="container">
                    {
                        if let Some(message) = &self.state.registration_success_message
                        {
                            html!
                            {
                                <>
                                <h3>{ message }</h3>
                                <Anchor route=AppRoute::SignInUser>
                                    <button class="button">{ "Sign in" }</button>
                                </Anchor>
                                </>
                            }
                        }
                        else
                        {
                            html!
                            {
                                <>
                                    <h3>{ "Register" }</h3>
                                    <input
                                        class="authentication_input_field" placeholder="user name"
                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditUserName(e.value)) />
                                    <input
                                        class="authentication_input_field" placeholder="email"
                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditEmail(e.value)) />
                                    <input
                                        class="authentication_input_field" type="password" placeholder="password"
                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditPassword(e.value)) />
                                    <button class="button" onclick=self.link.callback(|_| Msg::Submit)>{ "Submit" }</button>
                                    {
                                        if let Some(message) = &self.state.registration_error_message
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
