#![recursion_limit="512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

use yew::format::{Nothing, Json};
use anyhow::Error;

mod route;
mod components;
mod pages;
mod types;

use components::NavBar;
use pages::{HomePage, SignInUser, RegisterUser, UserInfo};
use yew_router::prelude::*;
use route::Route;
use types::AuthorizedUserResponse;


const KEY: &str = "authorization";

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;


struct State
{
    token: Option<String>,
    user: Option<AuthorizedUserResponse>,
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
    SaveToken(String),
    IdentifyUser(String),
    AuthorizedUser(Result<AuthorizedUserResponse, Error>),
    NotAuthorizedUser
}


impl Model
{
    fn save_token(&mut self, token: &str)
    {
        self.state.storage.store(KEY, Ok(token.to_string()));
    }


    fn identify_user(&self, token: &str) -> FetchTask
    {
        let callback: FetchCallback<AuthorizedUserResponse> = self.link.callback(
            move |response: FetchResponse<AuthorizedUserResponse>|
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
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let token =
            {
                if let Ok(token) = storage.restore(KEY)
                {
                    link.send_message(Msg::IdentifyUser(token.clone()));
                    Some(token.clone())
                }
                else
                {
                    None
                }
            };


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
            Msg::SaveToken(token) => self.save_token(&token),
            Msg::IdentifyUser(token) =>
                {
                    self.performing_task = true;
                    let task = self.identify_user(&token);
                    self.fetch_task = Some(task);

                },
            Msg::AuthorizedUser(response) =>
                {
                    self.performing_task = false;
                    self.state.user = response.ok();
                },
            Msg::NotAuthorizedUser =>
                {
                    self.performing_task = false;
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
        let user = self.state.user.clone();
        let token = self.state.token.clone();
        let handle_save_token = self.link.callback(|token: String| Msg::SaveToken(token));
        let handle_identify_user = self.link.callback(|token: String| Msg::IdentifyUser(token));

        let render = Router::render(move |switch: Route| match switch
        {
            Route::SignInUser => html! { <SignInUser
                                            user=user.clone()
                                            token=token.clone()
                                            save_token=handle_save_token.clone()
                                            identify_user=handle_identify_user.clone() /> },
            Route::RegisterUser => html! { <RegisterUser /> },
            Route::UserInfo => html! { <UserInfo /> },
            Route::HomePage => html! { <HomePage /> },
        });

        html!
        {
            <div>
                <NavBar user=self.state.user.clone() token=self.state.token.clone() />
                <Router<Route, ()> render=render />
            </div>
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app()
{
    App::<Model>::new().mount_to_body();
}
