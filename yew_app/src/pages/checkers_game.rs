use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use web_sys;

use yew::services::timeout::{TimeoutService, TimeoutTask};
use std::time::Duration;

use crate::types::{AuthorizedUserResponse, WsRequest, WsResponse, ChatMessageResponse};


use yew_router::{route::Route, switch::Permissive, Switch};
use yew_router::service::RouteService;
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
use yew_router;

use crate::route::AppRoute;


const INVITATION_WAITING_TIME: Duration = Duration::from_secs(10);
const WEBSOCKET_URL: &str = "ws://localhost:8080/ws/";
// const WEBSOCKET_URL: &str = "wss://gp.stresstable.com/ws/";


enum Actions
{
    UsersOnline,
    JoinToRoom,
    SetName,
    SendMessage,
    Invitation,
    AcceptInvitation,
    DeclineInvitation,
    UsersOnlineResponse,
}


impl Actions
{
    fn as_str(&self) -> String
    {
        match self
        {
            Actions::UsersOnline => String::from("users_online"),
            Actions::JoinToRoom => String::from("join_to_room"),
            Actions::SetName => String::from("users_online"),
            Actions::SendMessage => String::from("send_message"),
            Actions::Invitation => String::from("invitation"),
            Actions::AcceptInvitation => String::from("accept_invitation"),
            Actions::DeclineInvitation => String::from("decline_invitation"),
            Actions::UsersOnlineResponse => String::from("user_online_response"),
        }
    }
}


pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;


#[derive(Properties, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
}


struct ChatMessage(String);


struct OnlineUser(String);


struct SentInvitation
{
    to_user: String
}


struct ReceivedInvitation
{
    from_user: String
}


struct TimeoutTaskData
{
    timeout_task: TimeoutTask,
    received_invitation: ReceivedInvitation
}


struct State
{
    message: Option<String>,
    chat_messages: Vec<ChatMessage>,
    online_users: Vec<OnlineUser>,
    is_connected: bool,
    is_chat_room_defined: bool,
    sent_invitations: Vec<SentInvitation>,
    received_invitations: Vec<ReceivedInvitation>,
}


pub struct CheckersGame
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    websockets_task: Option<WebSocketTask>,
    fetch_task: Option<FetchTask>,
    timeout_tasks: Vec<TimeoutTaskData>,
}


impl From<WsAction> for Msg
{
    fn from(action: WsAction) -> Self
    {
        Msg::WsAction(action)
    }
}


pub enum WsAction
{
    Connect,
    SendData,
    Disconnect,
    Lost,
    SendInvitation(String),
    DeclineInvitation(String),
    AcceptInvitation(String),
}


pub enum Msg
{
    UpdateEditMessage(String),
    DefineButton(u32),
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    Ignore,
    ExtractChatLog,
    ChatLogReceived(Result<Vec<ChatMessageResponse>, Error>),
    ChatLogNotReceived,
    ShowAlert,
}


impl CheckersGame
{
    fn add_message_to_content(&mut self, message: String)
    {
        if message == "Someone disconnected" || message == "Someone connected"
        {
            if let Some(_) = &self.websockets_task
            {
                self.state.online_users = Vec::new();
                let online_users_request = WsRequest { action: "users_online".to_owned(), data: "".to_owned() };
                self.websockets_task.as_mut().unwrap().send(Json(&online_users_request));
            }

            let mut obsolete_received_invitations_indexes = Vec::new();
            for i in 0..self.state.received_invitations.len()
            {
                if let None = self.state.online_users
                    .iter()
                    .position(|online_user| self.state.received_invitations[i].from_user == online_user.0)
                {
                    obsolete_received_invitations_indexes.push(i)
                }
            }
            obsolete_indexes.sort();
            for idx in obsolete_received_invitations_indexes.iter().rev()
            {
                self.state.received_invitations.remove(*idx);
            }

        }
        else
        {
            self.state.chat_messages.push(ChatMessage(message));
        }
    }


    fn extract_chat_log(&self) -> FetchTask
    {
        let callback: FetchCallback<Vec<ChatMessageResponse>> = self.link.callback(
            move |response: FetchResponse<Vec<ChatMessageResponse>>|
                {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success()
                    {
                        Msg::ChatLogReceived(data)
                    }
                    else
                    {
                        Msg::ChatLogNotReceived
                    }
                },
        );
        let request = Request::get("/chat/extract_log/checkers_game")
            .body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }


    fn extract_chat_messages(&mut self, messages: Option<Vec<ChatMessageResponse>>)
    {
        if let Some(messages) = messages
        {
            for message in messages
            {
                let processed_message =
                    {
                        if let Some(user) = &self.props.user
                        {
                            if user.user_name == message.user_name
                            {
                                format!("you: {}", message.message)
                            }
                            else
                            {
                                format!("{}: {}", message.user_name, message.message)
                            }
                        }
                        else
                        {
                            format!("{}: {}", message.user_name, message.message)
                        }
                    };
                self.add_message_to_content(processed_message);
            }
        }
    }


    fn auto_decline_invitation(&mut self, from_user: String) -> TimeoutTask
    {
        let callback = self.link.callback(move |_| WsAction::DeclineInvitation(from_user.to_owned()));
        TimeoutService::spawn(INVITATION_WAITING_TIME, callback)
    }


    fn invitation_status_check(&self, to_user: String) -> bool
    {
        for invitation in &self.state.sent_invitations
        {
            if invitation.to_user == to_user
            {
                return true;
            }
        }
        false
    }
}


impl Component for CheckersGame
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
        {
            link, props,
            state: State
                {
                    message: None, chat_messages: Vec::new(), online_users: Vec::new(),
                    is_connected: false, is_chat_room_defined: false,
                    sent_invitations: Vec::new(), received_invitations: Vec::new(),
                },
            websockets_task: None, fetch_task: None, timeout_tasks: Vec::new()
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        if let Some(_) = &self.websockets_task
        {
            if !self.state.is_chat_room_defined
            {
                let join_to_room_request = WsRequest { action: "join_to_room".to_owned(), data: "checkers_game".to_owned() };
                self.websockets_task.as_mut().unwrap().send(Json(&join_to_room_request));

                if let Some(user) = &self.props.user
                {
                    let set_name_request = WsRequest { action: "set_name".to_owned(), data: format!("{}", user.user_name) };
                    self.websockets_task.as_mut().unwrap().send(Json(&set_name_request));

                    let online_users_request = WsRequest { action: "users_online".to_owned(), data: format!("{}", user.user_name) };
                    self.websockets_task.as_mut().unwrap().send(Json(&online_users_request));
                }
                self.state.is_chat_room_defined = true;
            }
        }

        match msg
        {
            Msg::UpdateEditMessage(e) => self.state.message = Some(e),
            Msg::DefineButton(key_code) =>
                {
                    if key_code == 13
                    {
                        self.link.send_message(WsAction::SendData);
                    }
                },
            Msg::WsAction(action) =>
                {
                    match action
                    {
                        WsAction::Connect =>
                            {
                                let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
                                let notification = self.link.callback(|status| match status
                                {
                                    WebSocketStatus::Opened => Msg::Ignore,
                                    WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                                });
                                let task =
                                    WebSocketService::connect(WEBSOCKET_URL, callback, notification)
                                        .unwrap();
                                self.websockets_task = Some(task);
                                self.state.is_connected = true;
                                self.state.is_chat_room_defined = false;
                            },
                        WsAction::SendData =>
                            {
                                if let Some(message) = &self.state.message.clone()
                                {
                                    if !message.is_empty()
                                    {
                                        if let Some(_) = &self.websockets_task
                                        {
                                            if let Some(_) = &self.props.user
                                            {
                                                self.add_message_to_content(format!("you: {}", message));
                                            }
                                            else
                                            {
                                                self.add_message_to_content(message.to_owned());
                                            }

                                            let request = WsRequest { action: "send_message".to_owned(), data: message.to_owned() };
                                            self.websockets_task.as_mut().unwrap().send(Json(&request));

                                            self.state.message = None;
                                        }
                                        else { return false; }
                                    }
                                    else { return false; }
                                }
                                else { return false; }
                            },
                        WsAction::SendInvitation(to_user) =>
                            {
                                self.state.sent_invitations.push(SentInvitation { to_user: to_user.to_owned() });
                                let request = WsRequest { action: "invitation".to_owned(), data: to_user.to_owned() };
                                self.websockets_task.as_mut().unwrap().send(Json(&request));
                            },
                        WsAction::DeclineInvitation(to_user) =>
                            {
                                if let Some(idx) = self.timeout_tasks.iter()
                                    .position(|data| data.received_invitation.from_user == to_user)
                                {
                                    self.timeout_tasks.remove(idx);
                                }

                                if let Some(idx) = self.state.received_invitations
                                    .iter()
                                    .position(|invitation| invitation.from_user == to_user)
                                {
                                    let dropped_invitation = self.state.received_invitations.remove(idx);
                                    let request = WsRequest { action: "decline_invitation".to_owned(), data: dropped_invitation.from_user.to_owned() };
                                    self.websockets_task.as_mut().unwrap().send(Json(&request));
                                }

                                // let route = Route::<()>::from(AppRoute::UserInfo);
                                // let mut router = RouteAgentDispatcher::new();
                                // router.send(RouteRequest::ChangeRoute(route));

                            },
                        WsAction::AcceptInvitation(to_user) =>
                            {
                                // if let Some(idx) = self.timeout_tasks.iter()
                                //     .position(|data| data.received_invitation.from_user == to_user)
                                // {
                                //     self.timeout_tasks.remove(idx);
                                // }

                                for data in &self.timeout_tasks
                                {
                                    let from_user = data.received_invitation.from_user.to_owned();
                                    if from_user != to_user
                                    {
                                        let request = WsRequest { action: "decline_invitation".to_owned(), data: from_user };
                                        self.websockets_task.as_mut().unwrap().send(Json(&request));
                                    }
                                }
                                self.timeout_tasks = Vec::new();
                                self.state.sent_invitations = Vec::new();
                                self.state.received_invitations = Vec::new();
                                let request = WsRequest { action: "accept_invitation".to_owned(), data: to_user.to_owned() };
                                self.websockets_task.as_mut().unwrap().send(Json(&request));


                                // if let Some(idx) = self.state.received_invitations
                                //     .iter()
                                //     .position(|invitation| invitation.from_user == to_user)
                                // {
                                //     let dropped_invitation = self.state.received_invitations.remove(idx);
                                //     let request = WsRequest { action: "accept_invitation".to_owned(), data: dropped_invitation.from_user.to_owned() };
                                //     self.websockets_task.as_mut().unwrap().send(Json(&request));
                                // }

                                if let Some(user) = &self.props.user
                                {
                                    let join_to_room_request = WsRequest
                                    {
                                        action: "join_to_room".to_owned(),
                                        data: format!("checkers_game_{}_{}", user.user_name, to_user),
                                    };
                                    self.websockets_task.as_mut().unwrap().send(Json(&join_to_room_request));
                                }
                            },
                        WsAction::Disconnect =>
                            {
                                for data in &self.timeout_tasks
                                {
                                    let from_user = data.received_invitation.from_user.to_owned();
                                    let request = WsRequest { action: "decline_invitation".to_owned(), data: from_user };
                                    self.websockets_task.as_mut().unwrap().send(Json(&request));
                                }
                                self.timeout_tasks = Vec::new();
                                self.state.sent_invitations = Vec::new();
                                self.state.received_invitations = Vec::new();

                                self.websockets_task.take();
                                self.state.is_connected = false;
                                self.state.online_users = Vec::new();
                            },
                        WsAction::Lost => self.websockets_task = None,
                    }
                },
            Msg::Ignore => return false,
            Msg::WsReady(response) =>
                {
                    // if let Some(received_data) = response.map(|data| data).ok()

                    if let Some(received_data) = response.ok()
                    {
                        if received_data.action == "users_online_response"
                        {
                            self.state.online_users.push(OnlineUser(received_data.data));
                        }
                        else if received_data.action == "invitation"
                        {
                            self.state.received_invitations.push(ReceivedInvitation { from_user: received_data.data.clone() });
                            let task = self.auto_decline_invitation(received_data.data.clone());
                            self.timeout_tasks.push(
                                TimeoutTaskData
                                    {
                                        timeout_task: task,
                                        received_invitation: ReceivedInvitation { from_user: received_data.data.clone() }
                                    }
                            );
                        }
                        else if received_data.action == "decline_invitation"
                        {
                            if let Some(idx) = self.state.sent_invitations
                                .iter()
                                .position(|invitation| invitation.to_user == received_data.data.clone())
                            {
                                self.state.sent_invitations.remove(idx);
                            }
                        }
                        else if received_data.action == "accept_invitation"
                        {
                            // if let Some(idx) = self.state.sent_invitations
                            //     .iter()
                            //     .position(|invitation| invitation.to_user == received_data.data.clone())
                            // {
                            //     self.state.sent_invitations.remove(idx);
                            // }

                            for data in &self.timeout_tasks
                            {
                                let from_user = data.received_invitation.from_user.to_owned();
                                if from_user != received_data.data
                                {
                                    let request = WsRequest { action: "decline_invitation".to_owned(), data: from_user };
                                    self.websockets_task.as_mut().unwrap().send(Json(&request));
                                }
                            }
                            self.timeout_tasks = Vec::new();
                            self.state.sent_invitations = Vec::new();
                            self.state.received_invitations = Vec::new();


                            // self.state.sent_invitations = Vec::new();
                            // for data in &self.timeout_tasks
                            // {
                            //     if data.received_invitation.from_user != received_data.data
                            //     {
                            //         let request = WsRequest { action: "decline_invitation".to_owned(), data: data.received_invitation.from_user.to_owned() };
                            //         self.websockets_task.as_mut().unwrap().send(Json(&request));
                            //     }
                            // }
                            // self.timeout_tasks = Vec::new();

                            if let Some(user) = &self.props.user
                            {
                                let join_to_room_request = WsRequest
                                {
                                    action: Actions::JoinToRoom.as_str(),
                                    data: format!("checkers_game_{}_{}", received_data.data.clone(), user.user_name),
                                };
                                self.websockets_task.as_mut().unwrap().send(Json(&join_to_room_request));
                            }
                        }
                        else
                        {
                            self.add_message_to_content(received_data.data);
                        }

                    }
                    else { return false; }
                },
            Msg::ExtractChatLog =>
                {
                    let task = self.extract_chat_log();
                    self.fetch_task = Some(task);
                },
            Msg::ChatLogReceived(response) =>
                {
                    self.extract_chat_messages(response.ok());
                },
            Msg::ChatLogNotReceived => return false,
            Msg::ShowAlert =>
                {
                    yew::services::ConsoleService::log("You should sign in and connect first.");
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
            <main class="main">
                <div class="container">
                    <h3>{ "Chat" }</h3>
                    <div>
                        {
                            if self.state.is_connected
                            {
                                html!
                                {
                                    <button onclick=self.link.callback(|_| WsAction::Disconnect)>{ "Disconnect" }</button>
                                }
                            }
                            else
                            {
                                html!
                                {
                                    <button onclick=self.link.callback(|_| WsAction::Connect)>{ "Connect" }</button>
                                }
                            }
                        }
                    </div>
                    <div id="checkers_game_chat_log" class="checkers_game_chat_log">
                        {
                            for self.state.chat_messages.iter().map(|chat_message: &ChatMessage|
                            {
                                html! { <> { &chat_message.0 } <br /> </>  }
                            })
                        }
                    </div>
                    <input
                        value=
                            {
                                if let Some(message) = &self.state.message
                                {
                                    message.to_string()
                                }
                                else
                                {
                                    "".to_string()
                                }
                            }
                        oninput=self.link.callback(|d: InputData| Msg::UpdateEditMessage(d.value))
                        onkeypress=self.link.callback(|e: KeyboardEvent| Msg::DefineButton(e.key_code()))
                    />
                    {
                        if let Some(_) = &self.props.user
                        {
                            if self.state.is_connected
                            {
                                html! { <button onclick=self.link.callback(|_| WsAction::SendData)>{ "Send" }</button> }
                            }
                            else
                            {
                                html! { <button onmouseover=self.link.callback(|_| Msg::ShowAlert)>{ "Send" }</button> }
                            }
                        }
                        else
                        {
                            html! { <button onmouseover=self.link.callback(|_| Msg::ShowAlert)>{ "Send" }</button> }
                        }
                    }

                    <h3>{ "Users online" }</h3>
                    <div class="checkers_game_online_users">
                        <table>
                            // <thead>
                            //     <tr>
                            //         <th>{ "User name" }</th>
                            //     </tr>
                            // </thead>
                            <tbody>
                            {
                                for self.state.online_users.iter().map(|online_user: &OnlineUser|
                                html!
                                {
                                    <tr>
                                        <td>{ &online_user.0 }</td>
                                        <td>
                                            {
                                                if true
                                                {
                                                    let user_name = online_user.0.to_owned();
                                                    html!
                                                    {
                                                        <button
                                                            onclick=self.link.callback(move |_| WsAction::SendInvitation(user_name.clone()))
                                                            disabled=self.invitation_status_check(user_name.clone())>
                                                            { "invite to play" }
                                                        </button>
                                                    }
                                                }
                                                else
                                                {
                                                    html! {  }
                                                }
                                            }
                                        </td>
                                    </tr>
                                })
                            }
                            </tbody>
                        </table>
                    </div>

                    <h3>{ "Invitations" }</h3>
                    <div class="checkers_game_invitation">
                        <table>
                            <tbody>
                            {
                                for self.state.received_invitations.iter().map(|invitation: &ReceivedInvitation|
                                html!
                                {
                                    <tr>
                                        <td>{ &invitation.from_user }</td>
                                        <td>
                                            {
                                                if true
                                                {
                                                    let user_name = invitation.from_user.to_owned();
                                                    html!
                                                    {
                                                        <button
                                                            onclick=self.link.callback(move |_| WsAction::AcceptInvitation(user_name.clone()))>
                                                            { "Accept" }
                                                        </button>
                                                    }
                                                }
                                                else
                                                {
                                                    html! {  }
                                                }
                                            }
                                        </td>
                                        <td>
                                            {
                                                if true
                                                {
                                                    let user_name = invitation.from_user.to_owned();
                                                    html!
                                                    {
                                                        <button
                                                            onclick=self.link.callback(move |_| WsAction::DeclineInvitation(user_name.clone()))>
                                                            { "Decline" }
                                                        </button>
                                                    }
                                                }
                                                else
                                                {
                                                    html! {  }
                                                }
                                            }
                                        </td>
                                    </tr>
                                })
                            }
                            </tbody>
                        </table>
                    </div>

                </div>
            </main>
        }
    }


    fn rendered(&mut self, first_render: bool)
    {
        if first_render
        {
            self.link.send_message(Msg::ExtractChatLog);
            // self.link.send_message(WsAction::Connect);
        }

        if let Some(element) = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("checkers_game_chat_log")
        {
            element.set_scroll_top(element.scroll_height());
        }
    }
}
