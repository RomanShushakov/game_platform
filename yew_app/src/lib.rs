#![recursion_limit="256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

use yew::format::{Nothing, Json};
use anyhow::Error;

// use serde::{Deserialize, Serialize};

mod route;
mod components;
mod pages;
mod types;

use components::NavBar;
use pages::{HomePage, SignInUser, RegisterUser, UserInfo};
use yew_router::prelude::*;
use route::Route;
use types::User;


const KEY: &str = "authorization";
const TOKEN: &str  = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX25hbWUiOiJrYW5vIiwiZW1haWwiOiJrYW5vQG1rLmNvbSIsImV4cCI6MTU5Nzk0NzczMn0.9WGnNX3gG6MGMgLDKN3cA9uINirDj1ApeG37ArNu25c";

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;


struct State
{
    token: Option<String>,
    user: Option<User>,
    storage: StorageService
}

struct Model
{
    link: ComponentLink<Self>,
    state: State,
    fetch_task: Option<FetchTask>,
    performing_task: bool
}


enum Msg
{
    IdentifyUser,
    AuthorizedUser(Result<User, Error>),
    NotAuthorizedUser,
    SaveToken
}


impl Model
{
    fn save_token(&mut self)
    {
        self.state.storage.store(KEY, Ok(TOKEN.to_string()));
    }


    fn identify_user(&self, token: &str) -> FetchTask
    {
        let callback: FetchCallback<User> = self.link.callback(
            move |response: FetchResponse<User>|
                {
                    let (meta, Json(user_data)) = response.into_parts();
                    if meta.status.is_success()
                    {
                        Msg::AuthorizedUser(user_data)
                    }
                    else
                    {
                        Msg::NotAuthorizedUser
                    }
                },
            );

        let request = Request::get("/auth/identify_user")
            .header(KEY, token)
            .body(Nothing)
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }

}


impl Component for Model
{
    type Message = Msg;
    type Properties = ();


    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        link.send_message(Msg::SaveToken);

        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let token =
            {
                if let Ok(token) = storage.restore(KEY)
                {
                    Some(token)
                }
                else
                {
                    None
                }
            };

        link.send_message(Msg::IdentifyUser);

        Self {
            link,
            state: State { storage, token, user: None },
            performing_task: false,
            fetch_task: None,
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::SaveToken => self.save_token(),
            Msg::IdentifyUser =>
                {
                    if let Some(token) = &self.state.token
                    {
                        self.performing_task = true;
                        let task = self.identify_user(token);
                        self.fetch_task = Some(task);
                    }
                    else
                    {
                        return false;
                    }
                },
            Msg::AuthorizedUser(response) =>
                {
                    self.performing_task = false;
                    self.state.user = response.ok();
                },
            Msg::NotAuthorizedUser =>
                {
                    yew::services::ConsoleService::log("not_authorized");
                    return false;
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
        let render = Router::render(move |switch: Route| match switch
        {
            Route::SignInUser => html! { <SignInUser /> },
            Route::RegisterUser => html! { <RegisterUser /> },
            Route::UserInfo => html! { <UserInfo /> },
            Route::HomePage => html! { <HomePage /> },
        });

        html!
        {
            <div>
                <NavBar user=self.state.user.clone() token=self.state.token.clone()/>
                <Router<Route, ()> render=render/>
            </div>
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app()
{
    App::<Model>::new().mount_to_body();
}
