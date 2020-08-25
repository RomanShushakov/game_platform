#![recursion_limit="1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

use yew::format::{Nothing, Json};
use anyhow::Error;

use yew_router::prelude::*;

mod route;
mod components;
mod pages;
mod types;

use components::NavBar;
use pages::{HomePage, SignInUser, RegisterUser, UserInfo};
use route::AppRoute;
use types::AuthorizedUserResponse;


pub const KEY: &str = "authorization";

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
}


enum Msg
{
    SaveToken(String),
    SignOut,
    IdentifyUser(String),
    AuthorizedUser(Result<AuthorizedUserResponse, Error>),
    NotAuthorizedUser
}


impl Model
{
    fn save_token(&mut self, token: &str)
    {
        self.state.token = Some(token.to_string());
        self.state.storage.store(KEY, Ok(token.to_string()));
    }


    fn sign_out(&mut self)
    {
        self.state.storage.remove(KEY);
        self.state.user = None;
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

        Self
        {
            link,
            state: State { storage, token, user: None },
            fetch_task: None
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::SaveToken(token) => self.save_token(&token),
            Msg::SignOut => self.sign_out(),
            Msg::IdentifyUser(token) =>
                {
                    let task = self.identify_user(&token);
                    self.fetch_task = Some(task);
                },
            Msg::AuthorizedUser(response) =>
                {
                    self.state.user = response.ok();
                },
            Msg::NotAuthorizedUser =>
                {
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

        let handle_sign_out = self.link.callback(|_| Msg::SignOut);

        let render = Router::render(move |switch: AppRoute| match switch
        {
            AppRoute::SignInUser => html! { <SignInUser
                                            user=user.clone()
                                            save_token=handle_save_token.clone()
                                            identify_user=handle_identify_user.clone() /> },
            AppRoute::RegisterUser => html! { <RegisterUser /> },
            AppRoute::UserInfo => html! { <UserInfo user=user.clone(), token=token.clone(), sign_out=handle_sign_out.clone() /> },
            AppRoute::HomePage => html! { <HomePage /> },
        });

        let handle_sign_out = self.link.callback(|_| Msg::SignOut);

        html!
        {
            <div>
                <NavBar user=self.state.user.clone(), token=self.state.token.clone(), sign_out=handle_sign_out.clone() />
                <Router<AppRoute, ()> render=render />
            </div>
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app()
{
    App::<Model>::new().mount_to_body();
}
