use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Nothing, Json};
use anyhow::Error;

use validator;

use crate::types::User;
use crate::KEY;


pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;


#[derive(Properties, Clone)]
pub struct Props
{
    pub token: Option<String>,
}


pub struct AllUsers
{
    link: ComponentLink<Self>,
    props: Props,
    users: Option<Vec<User>>,
    fetch_task: Option<FetchTask>,
}


pub enum Msg
{
    ShowAllUsers,
    Successful(Result<Vec<User>, Error>),
    Unsuccessful


}


impl AllUsers
{
    fn show_all_users(&self, token: &str) -> FetchTask
    {
        let callback: FetchCallback<Vec<User>> = self.link.callback(
            move |response: FetchResponse<Vec<User>>|
                {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success()
                    {
                        Msg::Successful(data)
                    }
                    else
                    {
                        Msg::Unsuccessful
                    }
                },
        );
        let request = Request::get("/auth/all_users")
            .header(KEY, token )
            .body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}


impl Component for AllUsers
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, users: None, fetch_task: None }
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
            Msg::Successful(response) => self.users = response.ok(),
            Msg::Unsuccessful => return false
        }
        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }


    fn view(&self) -> Html
    {
        html!
        {
            <>
                <div class="all_users_info">
                    <button onclick=self.link.callback(|_| Msg::ShowAllUsers)>{ "show all users" }</button>
                    <button disabled=true>{ "hide all users" }</button>
                </div>

                <div>
                    {
                        if let Some(users) = &self.users
                        {
                            html!
                            {
                                <>
                                    <h3>{ "All users" }</h3>
                                    <table id="tags_table" border="1" style="border-collapse: collapse;">
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
                                            for users.iter().map(|user: &User| html!
                                            {
                                                <tr>
                                                    <td>{ user.user_name.to_string() }</td>
                                                    <td>{ user.email.to_string() }</td>
                                                    <td>{ user.is_active.to_string() }</td>
                                                    <td>
                                                        {
                                                            if user.is_active
                                                            {
                                                                html! { <button>{ "deactivate" }</button> }
                                                            }
                                                            else
                                                            {
                                                                html! { <button>{ "activate" }</button> }
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
