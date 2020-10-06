use yew::prelude::*;
use yew::services::fetch::{FetchTask, Response, FetchService, Request};
use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::websocket::{WebSocketTask};


use web_sys;
use std::collections::HashSet;

use crate::types::
{
    AuthorizedUserResponse, WsRequest, ChatMessage, OnlineUser, SentInvitation, ChatMessageResponse,
    WsResponse
};
use crate::pages::{Actions};

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
    // pub online_users: HashSet<OnlineUser>,
    pub send_invitation: Callback<String>,
    pub sent_invitations: Vec<SentInvitation>,

    pub send_websocket_data: Callback<WsRequest>,
    pub websocket_chat_response: Option<WsResponse>,
    pub reset_websocket_chat_response: Callback<()>,

}



struct State
{
    message: Option<String>,
    chat_messages: Vec<ChatMessage>,
    online_users: HashSet<OnlineUser>,
    // is_connected: bool,
    // is_chat_room_defined: bool,
    // sent_invitations: Vec<SentInvitation>,
    // received_invitations: Vec<ReceivedInvitation>,
}


pub struct CheckersChat
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    fetch_task: Option<FetchTask>,
}


pub enum Msg
{
    Disconnect,
    Connect,
    UpdateEditMessage(String),
    DefineButton(u32),
    SendMessage,
    SendInvitation(String),
    ExtractChatLog,
    ChatLogReceived(Result<Vec<ChatMessageResponse>, Error>),
    ChatLogNotReceived,
}


impl CheckersChat
{
    fn invitation_status_check(&self, to_user: &str) -> bool
    {
        for invitation in &self.props.sent_invitations
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
            },
            fetch_task: None
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {

        match msg
        {
            Msg::Disconnect =>
                {
                    self.state.online_users = HashSet::new();
                    self.props.disconnect.emit(());
                },
            Msg::Connect =>
                {
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
                            let request = WsRequest { action: Actions::SendMessage.as_str(), data: message.to_string() };
                            self.props.send_websocket_data.emit(request);
                            self.state.message = None;
                        }
                        else { return false; }
                    }
                    else { return false; }

                },
            Msg::SendInvitation(user_name) => self.props.send_invitation.emit(user_name),
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
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            if let Some(response) = &self.props.websocket_chat_response
            {
                if response.action == Actions::ReceivedMessage.as_str()
                {
                    self.props.reset_websocket_chat_response.emit(());
                    self.state.chat_messages.push(ChatMessage(response.data.to_owned()));
                }
                else if response.action == Actions::SomeoneDisconnected.as_str()
                {
                    self.state.online_users = HashSet::new();
                    let online_users_request = WsRequest { action: Actions::RequestOnlineUsers.as_str(), data: "".to_string() };
                    self.props.reset_websocket_chat_response.emit(());
                    self.props.send_websocket_data.emit(online_users_request);
                }
                else if response.action == Actions::ResponseOnlineUsers.as_str()
                {
                    self.props.reset_websocket_chat_response.emit(());
                    self.state.online_users.insert(OnlineUser(response.data.clone()));
                }
                else if response.action == Actions::SomeoneConnected.as_str() // && response.data == "Someone connected"
                {
                    self.state.online_users = HashSet::new();
                    let online_users_request = WsRequest { action: Actions::RequestOnlineUsers.as_str(), data: "".to_string() };
                    self.props.reset_websocket_chat_response.emit(());
                    self.props.send_websocket_data.emit(online_users_request);
                }

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
            //
            //     <h3>{ "Invitations" }</h3>
            //     <div class="checkers_game_invitation">
            //         <table>
            //             <tbody>
            //             {
            //                 for self.state.received_invitations.iter().map(|invitation: &ReceivedInvitation|
            //                 html!
            //                 {
            //                     <tr>
            //                         <td>{ &invitation.from_user }</td>
            //                         <td>
            //                             {
            //                                 if true
            //                                 {
            //                                     let user_name = invitation.from_user.clone();
            //                                     html!
            //                                     {
            //                                         <button
            //                                             onclick=self.link.callback(move |_| WsAction::AcceptInvitation(user_name.clone()))>
            //                                             { "Accept" }
            //                                         </button>
            //                                     }
            //                                 }
            //                                 else
            //                                 {
            //                                     html! {  }
            //                                 }
            //                             }
            //                         </td>
            //                         <td>
            //                             {
            //                                 if true
            //                                 {
            //                                     let user_name = invitation.from_user.clone();
            //                                     html!
            //                                     {
            //                                         <button
            //                                             onclick=self.link.callback(move |_| WsAction::DeclineInvitation(user_name.clone()))>
            //                                             { "Decline" }
            //                                         </button>
            //                                     }
            //                                 }
            //                                 else
            //                                 {
            //                                     html! {  }
            //                                 }
            //                             }
            //                         </td>
            //                     </tr>
            //                 })
            //             }
            //             </tbody>
            //         </table>
            //     </div>
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