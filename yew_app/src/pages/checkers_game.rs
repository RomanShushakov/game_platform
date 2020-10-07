use yew::prelude::*;
use anyhow::Error;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use crate::types::{AuthorizedUserResponse, WsRequest, WsResponse};
use crate::components::CheckersBoard;
use crate::components::CheckersChat;


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


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
}


struct State
{
    is_connected: bool,
    is_chat_room_defined: bool,
    websocket_chat_response: Option<WsResponse>,
}


pub struct CheckersGame
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    websocket_task: Option<WebSocketTask>,
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
    ResetWebsocketChatResponse,
    Disconnect,
    Lost,
    MoveCheckersPiece(String),

}


pub enum Msg
{
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    Ignore,
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
                    is_connected: false,
                    is_chat_room_defined: false,
                    websocket_chat_response: None
                },
            websocket_task: None,
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
                        WsAction::Disconnect =>
                            {
                                self.state.websocket_chat_response = None;
                                self.websocket_task.take();
                                self.state.is_connected = false;
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
                    }
                    else { return false; }
                },
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
        let send_websocket_data_handle = self.link.callback(|request| Msg::WsAction(WsAction::SendWebSocketData(request)));
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
                                send_websocket_data=send_websocket_data_handle.clone(),
                                websocket_chat_response=self.state.websocket_chat_response.clone(),
                                reset_websocket_chat_response=reset_websocket_chat_response_handle.clone(),
                             />
                        </div>

                        {
                            html! { <CheckersBoard user=self.props.user.clone() move_checkers_piece=move_checkers_piece_handle.clone() /> }
                        }

                    </div>
                </div>
            </main>
        }
    }
}
