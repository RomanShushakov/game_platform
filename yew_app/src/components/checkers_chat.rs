use yew::prelude::*;
use yew::services::fetch::{FetchTask, Response, FetchService, Request};
use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::timeout::{TimeoutService, TimeoutTask};
use std::time::Duration;

use web_sys;
use std::collections::HashSet;

use crate::types::
{
    AuthorizedUserResponse, WsRequest, ChatMessage, OnlineUser, SentInvitation, ChatMessageResponse,
    WsResponse, ReceivedInvitation
};
use crate::pages::{ChatAction};


const INVITATION_WAITING_TIME: Duration = Duration::from_secs(30);
const CHAT_LOG_URL: &str = "/chat/extract_log/checkers_game";


pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
    pub is_connected: bool,
    pub disconnect: Callback<()>,
    pub connect: Callback<()>,
    pub send_websocket_data: Callback<WsRequest>,
    pub reset_websocket_chat_response: Callback<()>,
    pub websocket_chat_response: Option<WsResponse>,
}



struct State
{
    message: Option<String>,
    chat_messages: Vec<ChatMessage>,
    online_users: HashSet<OnlineUser>,
    sent_invitations: Vec<SentInvitation>,
    received_invitations: Vec<ReceivedInvitation>,
}


struct TimeoutTaskData
{
    timeout_task: TimeoutTask,
    received_invitation: ReceivedInvitation
}


pub struct CheckersChat
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    fetch_task: Option<FetchTask>,
    timeout_tasks: Vec<TimeoutTaskData>,
}


pub enum Msg
{
    Disconnect,
    Connect,
    UpdateEditMessage(String),
    DefineButton(u32),
    SendMessage,
    ExtractChatLog,
    ChatLogReceived(Result<Vec<ChatMessageResponse>, Error>),
    ChatLogNotReceived,
    SendInvitation(String),
    DeclineInvitation(String),
    AcceptInvitation(String),
}


impl CheckersChat
{
    fn invitation_status_check(&self, to_user: &str) -> bool
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
        let request = Request::get(CHAT_LOG_URL)
            .body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }


    fn put_messages_into_chat(&mut self, messages: Option<Vec<ChatMessageResponse>>)
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
                self.state.chat_messages.push(ChatMessage(processed_message));
            }
        }
    }


    fn auto_decline_invitation(&mut self, from_user: String) -> TimeoutTask
    {
        let callback = self.link.callback(move |_| Msg::DeclineInvitation(from_user.clone()));
        TimeoutService::spawn(INVITATION_WAITING_TIME, callback)
    }


    fn decline_invitations(&mut self, skip_user_name: &str)
    {
        for data in &self.timeout_tasks
        {
            let from_user = &data.received_invitation.from_user;
            if from_user != skip_user_name
            {
                let request = WsRequest { action: ChatAction::DeclineInvitation.as_str(), data: from_user.to_owned() };
                self.props.send_websocket_data.emit(request);
            }
        }
        self.timeout_tasks = Vec::new();
        self.state.sent_invitations = Vec::new();
        self.state.received_invitations = Vec::new();
    }


}


impl Component for CheckersChat
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
        {
            props,
            link,
            state: State
            {
                message: None, chat_messages: Vec::new(), online_users: HashSet::new(),
                sent_invitations: Vec::new(), received_invitations: Vec::new(),
            },
            fetch_task: None, timeout_tasks: Vec::new(),
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::Disconnect =>
                {
                    self.state.online_users = HashSet::new();

                    for data in &self.timeout_tasks
                    {
                        let from_user = data.received_invitation.from_user.clone();
                        let request = WsRequest { action: ChatAction::DeclineInvitation.as_str(), data: from_user };
                        self.props.send_websocket_data.emit(request);
                    }
                    self.timeout_tasks = Vec::new();
                    self.state.sent_invitations = Vec::new();
                    self.state.received_invitations = Vec::new();

                    self.props.disconnect.emit(());
                },
            Msg::Connect =>
                {
                    self.link.send_message(Msg::ExtractChatLog);
                    self.props.connect.emit(());
                },
            Msg::UpdateEditMessage(e) => self.state.message = Some(e),
            Msg::DefineButton(key_code) =>
                {
                    if key_code == 13
                    {
                        self.link.send_message(Msg::SendMessage);
                    }
                },
            Msg::SendMessage =>
                {
                    if let Some(message) = &self.state.message.clone()
                    {
                        if !message.is_empty()
                        {
                            self.state.chat_messages.push(ChatMessage(format!("you: {}", message)));
                            let request = WsRequest { action: ChatAction::SendMessage.as_str(), data: message.to_string() };
                            self.props.send_websocket_data.emit(request);
                            self.state.message = None;
                        }
                        else { return false; }
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
                    self.put_messages_into_chat(response.ok());
                },
            Msg::ChatLogNotReceived => return false,
            Msg::SendInvitation(to_user) =>
                {
                    self.state.sent_invitations.push(SentInvitation { to_user: to_user.clone() });
                    let request = WsRequest { action: ChatAction::Invitation.as_str(), data: to_user };
                    self.props.send_websocket_data.emit(request);
                },
            Msg::DeclineInvitation(to_user) =>
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
                        let request = WsRequest { action: ChatAction::DeclineInvitation.as_str(), data: dropped_invitation.from_user };
                        self.props.send_websocket_data.emit(request);
                    }
                },
            Msg::AcceptInvitation(to_user) =>
                {
                    self.decline_invitations(&to_user);
                    let request = WsRequest { action: ChatAction::AcceptInvitation.as_str(), data: to_user.clone() };
                    self.props.send_websocket_data.emit(request);

                    if let Some(user) = &self.props.user
                    {
                        let join_to_room_request = WsRequest
                        {
                            action: ChatAction::JoinToRoom.as_str(),
                            data: format!("checkers_game_{}_{}", user.user_name, to_user),
                        };
                        self.props.send_websocket_data.emit(join_to_room_request);
                    }
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            if let Some(response) = self.props.websocket_chat_response.clone()
            {
                if response.action == ChatAction::ReceivedMessage.as_str()
                {
                    self.props.reset_websocket_chat_response.emit(());
                    self.state.chat_messages.push(ChatMessage(response.data.to_owned()));
                }
                else if response.action == ChatAction::SomeoneDisconnected.as_str()
                {
                    self.state.online_users = HashSet::new();
                    let online_users_request = WsRequest { action: ChatAction::RequestOnlineUsers.as_str(), data: "".to_string() };
                    self.props.reset_websocket_chat_response.emit(());
                    self.props.send_websocket_data.emit(online_users_request);
                    if let Some(idx) = self.state.received_invitations
                        .iter()
                        .position(|invitation| invitation.from_user == response.data)
                    {
                        self.state.received_invitations.remove(idx);
                    }
                    if let Some(idx) = self.timeout_tasks.iter()
                        .position(|data| data.received_invitation.from_user == response.data)
                    {
                        self.timeout_tasks.remove(idx);
                    }
                }
                else if response.action == ChatAction::ResponseOnlineUsers.as_str()
                {
                    self.props.reset_websocket_chat_response.emit(());
                    self.state.online_users.insert(OnlineUser(response.data.clone()));
                }
                else if response.action == ChatAction::SomeoneConnected.as_str() // && response.data == "Someone connected"
                {
                    self.state.online_users = HashSet::new();
                    let online_users_request = WsRequest { action: ChatAction::RequestOnlineUsers.as_str(), data: "".to_string() };
                    self.props.reset_websocket_chat_response.emit(());
                    self.props.send_websocket_data.emit(online_users_request);
                }
                else if response.action == ChatAction::Invitation.as_str()
                {
                    self.props.reset_websocket_chat_response.emit(());
                    self.state.received_invitations.push(ReceivedInvitation { from_user: response.data.clone() });
                    let task = self.auto_decline_invitation(response.data.clone());
                    self.timeout_tasks.push(
                        TimeoutTaskData
                            {
                                timeout_task: task,
                                received_invitation: ReceivedInvitation { from_user: response.data.clone() }
                            }
                    );
                }
                else if response.action == ChatAction::DeclineInvitation.as_str()
                {
                    self.props.reset_websocket_chat_response.emit(());
                    if let Some(idx) = self.state.sent_invitations
                        .iter()
                        .position(|invitation| &invitation.to_user == &response.data)
                    {
                        self.state.sent_invitations.remove(idx);
                    }
                }
                else if response.action == ChatAction::AcceptInvitation.as_str()
                {
                    self.props.reset_websocket_chat_response.emit(());
                    self.decline_invitations(&response.data);
                    if let Some(user) = &self.props.user
                    {
                        let join_to_room_request = WsRequest
                        {
                            action: ChatAction::JoinToRoom.as_str(),
                            data: format!("checkers_game_{}_{}", &response.data, user.user_name),
                        };
                        self.props.send_websocket_data.emit(join_to_room_request);
                    }
                }
                else { return false; }
            }
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
            <>
                <h3>{ "Chat" }</h3>
                <div>
                    {
                        if self.props.is_connected
                        {
                            html!
                            {
                                <button onclick=self.link.callback(|_| Msg::Disconnect)>{ "Disconnect" }</button>
                            }
                        }
                        else
                        {
                            html!
                            {
                                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
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
                        html! { <button disabled=!self.props.is_connected onclick=self.link.callback(|_| Msg::SendMessage)>{ "Send" }</button> }
                    }
                    else
                    {
                        html! { html! { <button disabled=true>{ "Send" }</button> } }
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
                                                let user_name = online_user.0.clone();
                                                html!
                                                {
                                                    <button
                                                        onclick=self.link.callback(move |_| Msg::SendInvitation(user_name.clone()))
                                                        disabled=self.invitation_status_check(&user_name)>
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
                                                let user_name = invitation.from_user.clone();
                                                html!
                                                {
                                                    <button
                                                        onclick=self.link.callback(move |_| Msg::AcceptInvitation(user_name.clone()))>
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
                                                        onclick=self.link.callback(move |_| Msg::DeclineInvitation(user_name.clone()))>
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
            </>
        }
    }

    fn rendered(&mut self, first_render: bool)
    {
        if first_render
        {
            self.link.send_message(Msg::ExtractChatLog);
        }

        if let Some(element) = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("checkers_game_chat_log")
        {
            element.set_scroll_top(element.scroll_height());
        }
    }

}