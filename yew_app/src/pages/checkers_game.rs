use yew::prelude::*;
// use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use web_sys;

use yew::services::timeout::{TimeoutService, TimeoutTask};
use std::time::Duration;

use crate::types::
{
    AuthorizedUserResponse, WsRequest, WsResponse, ChatMessage, ChatMessageResponse,
    OnlineUser, SentInvitation
};


use yew_router::{route::Route, switch::Permissive, Switch};
use yew_router::service::RouteService;
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
use yew_router;

use crate::route::AppRoute;
use crate::components::CheckersBoard;
use crate::components::CheckersChat;


use std::collections::HashSet;
use crate::pages::checkers_game::WsAction::SendWebSocketData;


const INVITATION_WAITING_TIME: Duration = Duration::from_secs(30);
const WEBSOCKET_URL: &str = "ws://localhost:8080/ws/";
// const WEBSOCKET_URL: &str = "wss://gp.stresstable.com/ws/";
pub const GAME_NAME: &str = "checkers_game";


pub enum Actions
{
    RequestOnlineUsers,
    JoinToRoom,
    SetName,
    SendMessage,
    Invitation,
    AcceptInvitation,
    DeclineInvitation,
    ResponseOnlineUsers,
    SomeoneDisconnected,
    SomeoneConnected,

    ReceivedMessage,


}


impl Actions
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            Actions::RequestOnlineUsers => String::from("request_online_users"),
            Actions::JoinToRoom => String::from("join_to_room"),
            Actions::SetName => String::from("set_name"),
            Actions::SendMessage => String::from("send_message"),
            Actions::Invitation => String::from("invitation"),
            Actions::AcceptInvitation => String::from("accept_invitation"),
            Actions::DeclineInvitation => String::from("decline_invitation"),
            Actions::ResponseOnlineUsers => String::from("response_online_users"),
            Actions::SomeoneDisconnected => String::from("disconnect"),
            Actions::SomeoneConnected => String::from("connect"),

            Actions::ReceivedMessage => String::from("received_message"),
        }
    }
}


// pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
// type FetchCallback<T> = Callback<FetchResponse<T>>;


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
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
    // message: Option<String>,
    // chat_messages: Vec<ChatMessage>,
    // online_users: HashSet<OnlineUser>,
    is_connected: bool,
    is_chat_room_defined: bool,
    sent_invitations: Vec<SentInvitation>,
    received_invitations: Vec<ReceivedInvitation>,

    websocket_chat_response: Option<WsResponse>,
}


pub struct CheckersGame
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,

    websocket_task: Option<WebSocketTask>,
    // fetch_task: Option<FetchTask>,
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
    SendWebSocketData(WsRequest),
    // SendData,
    // SendMessage(WsRequest),
    Disconnect,
    Lost,
    SendInvitation(String),
    DeclineInvitation(String),
    AcceptInvitation(String),
    MoveCheckersPiece(String),

    ResetWebsocketChatResponse,

}


pub enum Msg
{
    // UpdateEditMessage(String),
    // DefineButton(u32),
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    Ignore,
    // ExtractChatLog,
    // ChatLogReceived(Result<Vec<ChatMessageResponse>, Error>),
    // ChatLogNotReceived,
}


impl CheckersGame
{
    // fn refresh_online_users_list(&mut self)
    // {
    //     if let Some(_) = &self.websocket_task
    //     {
    //         self.state.online_users = HashSet::new();
    //         let online_users_request = WsRequest { action: Actions::RequestOnlineUsers.as_str(), data: "".to_string() };
    //         self.websocket_task.as_mut().unwrap().send(Json(&online_users_request));
    //     }
    // }


    // fn add_message_to_content(&mut self, message: &str)
    // {
    //     if message == "Someone connected"
    //     {
    //         self.refresh_online_users_list();
    //     }
    //     else
    //     {
    //         self.state.chat_messages.push(ChatMessage(message.to_string()));
    //     }
    // }


    // fn extract_chat_log(&self) -> FetchTask
    // {
    //     let callback: FetchCallback<Vec<ChatMessageResponse>> = self.link.callback(
    //         move |response: FetchResponse<Vec<ChatMessageResponse>>|
    //             {
    //                 let (meta, Json(data)) = response.into_parts();
    //                 if meta.status.is_success()
    //                 {
    //                     Msg::ChatLogReceived(data)
    //                 }
    //                 else
    //                 {
    //                     Msg::ChatLogNotReceived
    //                 }
    //             },
    //     );
    //     let request = Request::get("/chat/extract_log/checkers_game")
    //         .body(Nothing).unwrap();
    //     FetchService::fetch(request, callback).unwrap()
    // }


    // fn extract_chat_messages(&mut self, messages: Option<Vec<ChatMessageResponse>>)
    // {
    //     if let Some(messages) = messages
    //     {
    //         for message in messages
    //         {
    //             let processed_message =
    //                 {
    //                     if let Some(user) = &self.props.user
    //                     {
    //                         if user.user_name == message.user_name
    //                         {
    //                             format!("you: {}", message.message)
    //                         }
    //                         else
    //                         {
    //                             format!("{}: {}", message.user_name, message.message)
    //                         }
    //                     }
    //                     else
    //                     {
    //                         format!("{}: {}", message.user_name, message.message)
    //                     }
    //                 };
    //             // self.link.callback(|message| Msg::ReceiveMessage(message)).emit(processed_message);
    //             // callback.emit(processed_message);
    //             self.add_message_to_content(&processed_message);
    //         }
    //     }
    // }


    fn auto_decline_invitation(&mut self, from_user: String) -> TimeoutTask
    {
        let callback = self.link.callback(move |_| WsAction::DeclineInvitation(from_user.clone()));
        TimeoutService::spawn(INVITATION_WAITING_TIME, callback)
    }


    // fn invitation_status_check(&self, to_user: &str) -> bool
    // {
    //     for invitation in &self.state.sent_invitations
    //     {
    //         if invitation.to_user == to_user
    //         {
    //             return true;
    //         }
    //     }
    //     false
    // }


    fn decline_invitations(&mut self, skip_user_name: &str)
    {
        for data in &self.timeout_tasks
        {
            let from_user = &data.received_invitation.from_user;
            if from_user != skip_user_name
            {
                let request = WsRequest { action: Actions::DeclineInvitation.as_str(), data: from_user.to_owned() };
                self.websocket_task.as_mut().unwrap().send(Json(&request));
            }
        }
        self.timeout_tasks = Vec::new();
        self.state.sent_invitations = Vec::new();
        self.state.received_invitations = Vec::new();
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
                    // message: None,
                    // chat_messages: Vec::new(),
                    // online_users: HashSet::new(),
                    is_connected: false,
                    is_chat_room_defined: false,
                    sent_invitations: Vec::new(),
                    received_invitations: Vec::new(),

                    websocket_chat_response: None

                },
            websocket_task: None,
            // fetch_task: None,
            timeout_tasks: Vec::new()
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        if let Some(_) = &self.websocket_task
        {
            if !self.state.is_chat_room_defined
            {
                let join_to_room_request = WsRequest { action: Actions::JoinToRoom.as_str(), data: GAME_NAME.to_string() };
                self.websocket_task.as_mut().unwrap().send(Json(&join_to_room_request));

                if let Some(user) = &self.props.user
                {
                    let set_name_request = WsRequest { action: Actions::SetName.as_str(), data: format!("{}", user.user_name) };
                    self.websocket_task.as_mut().unwrap().send(Json(&set_name_request));

                    let request_online_users = WsRequest { action: Actions::RequestOnlineUsers.as_str(), data: format!("{}", user.user_name) };
                    self.websocket_task.as_mut().unwrap().send(Json(&request_online_users));
                }
                self.state.is_chat_room_defined = true;
            }
        }

        match msg
        {
            // Msg::UpdateEditMessage(e) => self.state.message = Some(e),
            // Msg::DefineButton(key_code) =>
            //     {
            //         if key_code == 13
            //         {
            //             self.link.send_message(WsAction::SendData);
            //         }
            //     },
            Msg::WsAction(action) =>
                {
                    match action
                    {
                        WsAction::MoveCheckersPiece(position) => (),
                        WsAction::Connect =>
                            {
                                self.state.websocket_chat_response = None;

                                let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
                                let notification = self.link.callback(|status| match status
                                {
                                    WebSocketStatus::Opened => Msg::Ignore,
                                    WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                                });
                                let task =
                                    WebSocketService::connect(WEBSOCKET_URL, callback, notification)
                                        .unwrap();
                                self.websocket_task = Some(task);
                                self.state.is_connected = true;
                                self.state.is_chat_room_defined = false;
                            },
                        WsAction::SendWebSocketData(request) =>
                            {
                                if let Some(_) = &self.websocket_task
                                {
                                    self.websocket_task.as_mut().unwrap().send(Json(&request));
                                }
                                else { return false; }
                            },
                        WsAction::ResetWebsocketChatResponse => self.state.websocket_chat_response = None,

                        // WsAction::SendData =>
                        //     {
                        //         if let Some(message) = &self.state.message.clone()
                        //         {
                        //             if !message.is_empty()
                        //             {
                        //                 if let Some(_) = &self.websockets_task
                        //                 {
                        //                     if let Some(_) = &self.props.user
                        //                     {
                        //                         self.add_message_to_content(&format!("you: {}", message));
                        //                     }
                        //                     else
                        //                     {
                        //                         self.add_message_to_content(message);
                        //                     }
                        //
                        //                     let request = WsRequest { action: Actions::SendMessage.as_str(), data: message.to_string() };
                        //                     self.websockets_task.as_mut().unwrap().send(Json(&request));
                        //
                        //                     self.state.message = None;
                        //                 }
                        //                 else { return false; }
                        //             }
                        //             else { return false; }
                        //         }
                        //         else { return false; }
                        //     },
                        // WsAction::SendMessage(request) =>
                        //     {
                        //         if let Some(_) = &self.websocket_task
                        //         {
                        //             self.state.chat_messages.push(ChatMessage(format!("you: {}", request.data)));
                        //             let request = WsRequest { action: Actions::SendMessage.as_str(), data: request.data.to_string() };
                        //             self.websocket_task.as_mut().unwrap().send(Json(&request));
                        //         }
                        //         else { return false; }
                        //     },
                        WsAction::SendInvitation(to_user) =>
                            {
                                self.state.sent_invitations.push(SentInvitation { to_user: to_user.clone() });
                                let request = WsRequest { action: Actions::Invitation.as_str(), data: to_user };
                                self.websocket_task.as_mut().unwrap().send(Json(&request));
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
                                    let request = WsRequest { action: Actions::DeclineInvitation.as_str(), data: dropped_invitation.from_user };
                                    self.websocket_task.as_mut().unwrap().send(Json(&request));
                                }

                                // let route = Route::<()>::from(AppRoute::UserInfo);
                                // let mut router = RouteAgentDispatcher::new();
                                // router.send(RouteRequest::ChangeRoute(route));

                            },
                        WsAction::AcceptInvitation(to_user) =>
                            {
                                self.decline_invitations(&to_user);
                                let request = WsRequest { action: Actions::AcceptInvitation.as_str(), data: to_user.clone() };
                                self.websocket_task.as_mut().unwrap().send(Json(&request));

                                if let Some(user) = &self.props.user
                                {
                                    let join_to_room_request = WsRequest
                                    {
                                        action: Actions::JoinToRoom.as_str(),
                                        data: format!("checkers_game_{}_{}", user.user_name, to_user),
                                    };
                                    self.websocket_task.as_mut().unwrap().send(Json(&join_to_room_request));
                                }
                            },
                        WsAction::Disconnect =>
                            {
                                self.state.websocket_chat_response = None;

                                for data in &self.timeout_tasks
                                {
                                    let from_user = data.received_invitation.from_user.clone();
                                    let request = WsRequest { action: Actions::DeclineInvitation.as_str(), data: from_user };
                                    self.websocket_task.as_mut().unwrap().send(Json(&request));
                                }
                                self.timeout_tasks = Vec::new();
                                self.state.sent_invitations = Vec::new();
                                self.state.received_invitations = Vec::new();

                                self.websocket_task.take();
                                self.state.is_connected = false;
                                // self.state.online_users = HashSet::new();
                            },
                        WsAction::Lost => self.websocket_task = None,
                    }
                },
            Msg::Ignore => return false,
            Msg::WsReady(response) =>
                {

                    // if let Some(received_data) = response.map(|data| data).ok()
                    if let Some(received_data) = response.ok()
                    {
                        self.state.websocket_chat_response = Some(received_data.clone());


                        // if received_data.action == Actions::ResponseOnlineUsers.as_str()
                        // {
                        //     self.state.online_users.insert(OnlineUser(received_data.data.clone()));
                        // }


                        if received_data.action == Actions::Invitation.as_str()
                        {
                            self.state.received_invitations.push(ReceivedInvitation { from_user: received_data.data.clone() });
                            let task = self.auto_decline_invitation(received_data.data.clone());
                            self.timeout_tasks.push(
                                TimeoutTaskData
                                    {
                                        timeout_task: task,
                                        received_invitation: ReceivedInvitation { from_user: received_data.data }
                                    }
                            );
                        }
                        else if received_data.action == Actions::DeclineInvitation.as_str()
                        {
                            if let Some(idx) = self.state.sent_invitations
                                .iter()
                                .position(|invitation| &invitation.to_user == &received_data.data)
                            {
                                self.state.sent_invitations.remove(idx);
                            }
                        }
                        else if received_data.action == Actions::AcceptInvitation.as_str()
                        {
                            self.decline_invitations(&received_data.data);

                            if let Some(user) = &self.props.user
                            {
                                let join_to_room_request = WsRequest
                                {
                                    action: Actions::JoinToRoom.as_str(),
                                    data: format!("checkers_game_{}_{}", &received_data.data, user.user_name),
                                };
                                self.websocket_task.as_mut().unwrap().send(Json(&join_to_room_request));
                            }
                        }
                        else if received_data.action == Actions::SomeoneDisconnected.as_str()
                        {
                            // self.refresh_online_users_list();

                            if let Some(idx) = self.state.received_invitations
                                .iter()
                                .position(|invitation| invitation.from_user == received_data.data)
                            {
                                self.state.received_invitations.remove(idx);
                            }

                            if let Some(idx) = self.timeout_tasks.iter()
                                .position(|data| data.received_invitation.from_user == received_data.data)
                            {
                                self.timeout_tasks.remove(idx);
                            }
                        }
                        // else
                        // {
                        //     self.add_message_to_content(&received_data.data);
                        // }
                    }
                    else { return false; }
                },
            // Msg::ExtractChatLog =>
            //     {
            //         let task = self.extract_chat_log();
            //         self.fetch_task = Some(task);
            //     },
            // Msg::ChatLogReceived(response) =>
            //     {
            //         self.extract_chat_messages(response.ok());
            //     },
            // Msg::ChatLogNotReceived => return false,
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
        let move_checkers_piece_handle = self.link.callback(|data| Msg::WsAction(WsAction::MoveCheckersPiece(data)));

        let disconnect_handle = self.link.callback(|_| Msg::WsAction(WsAction::Disconnect));
        let connect_handle = self.link.callback(|_| Msg::WsAction(WsAction::Connect));


        let send_invitation_handle = self.link.callback(|to_user| Msg::WsAction(WsAction::SendInvitation(to_user)));


        let send_websocket_data_handle = self.link.callback(|request| Msg::WsAction(SendWebSocketData(request)));
        let reset_websocket_chat_response_handle = self.link.callback(|_| Msg::WsAction(WsAction::ResetWebsocketChatResponse));


        html!
        {
            <main class="main">
                <div class="container">
                    <div class="field">
                        <div class="container">
                            < CheckersChat
                                user=self.props.user.clone(),
                                is_connected=self.state.is_connected.clone(),
                                disconnect=disconnect_handle.clone(),
                                connect=connect_handle.clone(),


                                send_invitation=send_invitation_handle.clone(),
                                sent_invitations=self.state.sent_invitations.clone(),


                                send_websocket_data=send_websocket_data_handle.clone(),
                                reset_websocket_chat_response=reset_websocket_chat_response_handle.clone(),
                                websocket_chat_response=self.state.websocket_chat_response.clone(),


                             />
                            // <h3>{ "Chat" }</h3>
                            // <div>
                            //     {
                            //         if self.state.is_connected
                            //         {
                            //             html!
                            //             {
                            //                 <button onclick=self.link.callback(|_| WsAction::Disconnect)>{ "Disconnect" }</button>
                            //             }
                            //         }
                            //         else
                            //         {
                            //             html!
                            //             {
                            //                 <button onclick=self.link.callback(|_| WsAction::Connect)>{ "Connect" }</button>
                            //             }
                            //         }
                            //     }
                            // </div>
                            // <div id="checkers_game_chat_log" class="checkers_game_chat_log">
                            //     {
                            //         for self.state.chat_messages.iter().map(|chat_message: &ChatMessage|
                            //         {
                            //             html! { <> { &chat_message.0 } <br /> </>  }
                            //         })
                            //     }
                            // </div>
                            // <input
                            //     value=
                            //         {
                            //             if let Some(message) = &self.state.message
                            //             {
                            //                 message.to_string()
                            //             }
                            //             else
                            //             {
                            //                 "".to_string()
                            //             }
                            //         }
                            //     oninput=self.link.callback(|d: InputData| Msg::UpdateEditMessage(d.value))
                            //     onkeypress=self.link.callback(|e: KeyboardEvent| Msg::DefineButton(e.key_code()))
                            // />
                            // {
                            //     if let Some(_) = &self.props.user
                            //     {
                            //         if self.state.is_connected
                            //         {
                            //             html! { <button onclick=self.link.callback(|_| WsAction::SendData)>{ "Send" }</button> }
                            //         }
                            //         else
                            //         {
                            //             html! { <button onmouseover=self.link.callback(|_| Msg::ShowAlert)>{ "Send" }</button> }
                            //         }
                            //     }
                            //     else
                            //     {
                            //         html! { <button onmouseover=self.link.callback(|_| Msg::ShowAlert)>{ "Send" }</button> }
                            //     }
                            // }

                            // <h3>{ "Users online" }</h3>
                            // <div class="checkers_game_online_users">
                            //     <table>
                            //         // <thead>
                            //         //     <tr>
                            //         //         <th>{ "User name" }</th>
                            //         //     </tr>
                            //         // </thead>
                            //         <tbody>
                            //         {
                            //             for self.state.online_users.iter().map(|online_user: &OnlineUser|
                            //             html!
                            //             {
                            //                 <tr>
                            //                     <td>{ &online_user.0 }</td>
                            //                     <td>
                            //                         {
                            //                             if true
                            //                             {
                            //                                 let user_name = online_user.0.clone();
                            //                                 html!
                            //                                 {
                            //                                     <button
                            //                                         onclick=self.link.callback(move |_| WsAction::SendInvitation(user_name.clone()))
                            //                                         disabled=self.invitation_status_check(&user_name)>
                            //                                         { "invite to play" }
                            //                                     </button>
                            //                                 }
                            //                             }
                            //                             else
                            //                             {
                            //                                 html! {  }
                            //                             }
                            //                         }
                            //                     </td>
                            //                 </tr>
                            //             })
                            //         }
                            //         </tbody>
                            //     </table>
                            // </div>

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
                                                            let user_name = invitation.from_user.clone();
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
                                                            let user_name = invitation.from_user.clone();
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

                        {
                            html! { <CheckersBoard user=self.props.user.clone() move_checkers_piece=move_checkers_piece_handle.clone() /> }
                        }

                    </div>
                </div>
            </main>
        }
    }


    // fn rendered(&mut self, first_render: bool)
    // {
    //     if first_render
    //     {
    //         self.link.send_message(Msg::ExtractChatLog);
    //         // self.link.send_message(WsAction::Connect);
    //     }
    //
    //     if let Some(element) = web_sys::window().unwrap()
    //         .document().unwrap()
    //         .get_element_by_id("checkers_game_chat_log")
    //     {
    //         element.set_scroll_top(element.scroll_height());
    //     }
    // }
}
