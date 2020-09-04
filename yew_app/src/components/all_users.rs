use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Nothing, Json};
use anyhow::Error;

use crate::types::{UserForAllUsersResponse, UserChangeStatusRequest};
use crate::KEY;


pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;


#[derive(Properties, Clone)]
pub struct Props
{
    pub token: Option<String>,
}


struct State
{
    users: Option<Vec<UserForAllUsersResponse>>,
}


pub struct AllUsers
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    fetch_task: Option<FetchTask>,
}


pub enum Msg
{
    ShowAllUsers,
    UsersListReceived(Result<Vec<UserForAllUsersResponse>, Error>),
    UsersListNotReceived,
    HideAllUsers,
    ChangeUserStatus(String),
    UserStatusChanged(Result<String, Error>),
    UserStatusNotChanged(Result<String, Error>),
}


impl AllUsers
{
    fn show_all_users(&self, token: &str) -> FetchTask
    {
        let callback: FetchCallback<Vec<UserForAllUsersResponse>> = self.link.callback(
            move |response: FetchResponse<Vec<UserForAllUsersResponse>>|
                {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success()
                    {
                        Msg::UsersListReceived(data)
                    }
                    else
                    {
                        Msg::UsersListNotReceived
                    }
                },
        );
        let request = Request::get("/auth/all_users")
            .header(KEY, token )
            .body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }


    fn change_user_status(&self, token: &str, user_change_status_data: UserChangeStatusRequest) -> FetchTask
    {
        let callback = self.link.callback(
            move |response: Response<Result<String, Error>>|
                {
                    let (meta, data) = response.into_parts();
                    if meta.status.is_success()
                    {
                        Msg::UserStatusChanged(data)
                    }
                    else
                    {
                        Msg::UserStatusNotChanged(data)
                    }
                },
        );
        let request = Request::post("/auth/change_user_status")
            .header("Content-Type", "application/json")
            .header(KEY, token )
            .body(Json(&user_change_status_data)).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}


impl Component for AllUsers
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, state: State { users: None }, fetch_task: None }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowAllUsers =>
                {
                    if let Some(token) = &self.props.token
                    {
                        let task = self.show_all_users(token);
                        self.fetch_task = Some(task);
                    }
                    else { return false }
                },
            Msg::UsersListReceived(response) => self.state.users = response.ok(),
            Msg::UsersListNotReceived => return false,
            Msg::HideAllUsers => self.state.users = None,
            Msg::ChangeUserStatus(uid) =>
                {
                    if let Some(token) = &self.props.token
                    {
                        let user_change_status_data = UserChangeStatusRequest { uid };
                        let task = self.change_user_status(token, user_change_status_data);
                        self.fetch_task = Some(task);
                    }
                    else { return false }
                },
            Msg::UserStatusChanged(response) =>
                {
                    yew::services::dialog::DialogService::alert(&response.unwrap());
                    self.link.send_message(Msg::ShowAllUsers);
                },
            Msg::UserStatusNotChanged(response) =>
                {
                    yew::services::ConsoleService::log(&response.unwrap());
                    return false
                }
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        self.props = props;
        true
    }


    fn view(&self) -> Html
    {
        html!
        {
            <>
                <div class="all_users_info">
                    <button onclick=self.link.callback(|_| Msg::ShowAllUsers)>{ "show all users" }</button>
                    <button onclick=self.link.callback(|_| Msg::HideAllUsers)>{ "hide all users" }</button>
                </div>

                <div>
                    {
                        if let Some(users) = &self.state.users
                        {
                            html!
                            {
                                <>
                                    <h3>{ "All users" }</h3>
                                    <table border="1" style="border-collapse: collapse;">
                                        <thead>
                                            <tr>
                                                <th>{ "User name" }</th>
                                                <th>{ "Email" }</th>
                                                <th>{ "Is active" }</th>
                                                <th>{ "Action" }</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                        {
                                            for users.iter().map(|user: &UserForAllUsersResponse|
                                            html!
                                            {
                                                <tr>
                                                    <td>{ user.user_name.to_string() }</td>
                                                    <td>{ user.email.to_string() }</td>
                                                    <td>{ user.is_active.to_string() }</td>
                                                    <td>
                                                        {
                                                            if user.is_active
                                                            {
                                                                let uid = user.id.to_string();
                                                                html!
                                                                {
                                                                    <button onclick=self.link.callback(move |_| Msg::ChangeUserStatus(uid.clone()))>
                                                                        { "deactivate" }
                                                                    </button>
                                                                }
                                                            }
                                                            else
                                                            {
                                                                let uid = user.id.to_string();
                                                                html!
                                                                {
                                                                    <button onclick=self.link.callback(move |_| Msg::ChangeUserStatus(uid.clone()))>
                                                                        { "activate" }
                                                                    </button>
                                                                }
                                                            }
                                                        }
                                                    </td>
                                                </tr>
                                            })
                                        }
                                        </tbody>
                                    </table>
                                </>
                            }
                        }
                        else
                        {
                            html! { }
                        }
                    }
                </div>
            </>
        }
    }
}
