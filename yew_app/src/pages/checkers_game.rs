use yew::prelude::*;
use anyhow::Error;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use std::rc::Rc;

use crate::types::{AuthorizedUserResponse, WsRequest, WsResponse, PieceColor};
use crate::components::CheckersBoard;
use crate::components::CheckersChat;

use std::slice::Iter;
use self::ChatAction::*;
use self::GameAction::*;

use dotenv_codegen::dotenv;


pub const GAME_NAME: &str = "checkers_game";
pub const WEBSOCKET_URL: &str = dotenv!("WEBSOCKET_URL");


pub enum ChatAction
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


impl ChatAction
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            ChatAction::RequestOnlineUsers => String::from("request_online_users"),
            ChatAction::JoinToRoom => String::from("join_to_room"),
            ChatAction::SetName => String::from("set_name"),
            ChatAction::SendMessage => String::from("send_message"),
            ChatAction::Invitation => String::from("invitation"),
            ChatAction::AcceptInvitation => String::from("accept_invitation"),
            ChatAction::DeclineInvitation => String::from("decline_invitation"),
            ChatAction::ResponseOnlineUsers => String::from("response_online_users"),
            ChatAction::SomeoneDisconnected => String::from("disconnect"),
            ChatAction::SomeoneConnected => String::from("connect"),
            ChatAction::ReceivedMessage => String::from("received_message"),
        }
    }

    pub fn iterator() -> Iter<'static, ChatAction>
     {
        static ACTIONS: [ChatAction; 11] =
            [
                RequestOnlineUsers, JoinToRoom, SetName, SendMessage, Invitation,
                AcceptInvitation, DeclineInvitation, ResponseOnlineUsers,
                SomeoneDisconnected, SomeoneConnected, ReceivedMessage
            ];
        ACTIONS.iter()
    }
}


pub enum GameAction
{
    SendCheckerPieceMove,
    ReceivedCheckerPieceMove,
    SendLeaveGameMessage,
    ReceivedLeaveGameMessage,
}


impl GameAction
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            GameAction::SendCheckerPieceMove => String::from("send_checker_piece_move"),
            GameAction::ReceivedCheckerPieceMove => String::from("received_checker_piece_move"),
            GameAction::SendLeaveGameMessage => String::from("send_leave_game_message"),
            GameAction::ReceivedLeaveGameMessage => String::from("received_leave_game_message"),
        }
    }

    pub fn iterator() -> Iter<'static, GameAction>
     {
        static ACTIONS: [GameAction; 4] =
            [
                SendCheckerPieceMove, ReceivedCheckerPieceMove, SendLeaveGameMessage,
                ReceivedLeaveGameMessage
            ];
        ACTIONS.iter()
    }
}


#[derive(Properties, Clone)]
pub struct Props
{
    pub user: Rc<Option<AuthorizedUserResponse>>,
}


struct State
{
    is_connected: bool,
    is_chat_room_defined: bool,
    websocket_chat_response: Option<WsResponse>,
    is_in_game: bool,
    piece_color: Option<PieceColor>,
    websocket_game_response: Option<WsResponse>,
}


pub struct CheckersGame
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    websocket_task: Option<WebSocketTask>,
}


pub enum WsAction
{
    Connect,
    SendWebSocketData(WsRequest),
    ResetWebsocketChatResponse,
    Disconnect,
    Lost,
    StartGame,
    ChooseWhiteColor,
    ChooseBlackColor,
    ResetWebsocketGameResponse,
    LeaveGame,
}


pub enum Msg
{
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    Ignore,
}


impl From<WsAction> for Msg
{
    fn from(action: WsAction) -> Self
    {
        Msg::WsAction(action)
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
                    is_connected: false,
                    is_chat_room_defined: false,
                    websocket_chat_response: None,
                    is_in_game: false,
                    piece_color: None,
                    websocket_game_response: None,
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
                let join_to_room_request = WsRequest { action: ChatAction::JoinToRoom.as_str(), data: GAME_NAME.to_owned() };
                self.websocket_task.as_mut().unwrap().send(Json(&join_to_room_request));
                if let Some(user) = &*self.props.user
                {
                    let set_name_request = WsRequest { action: ChatAction::SetName.as_str(), data: user.user_name.to_owned() };
                    self.websocket_task.as_mut().unwrap().send(Json(&set_name_request));
                    let request_online_users = WsRequest { action: ChatAction::RequestOnlineUsers.as_str(), data: user.user_name.to_owned() };
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
                        WsAction::ResetWebsocketGameResponse => self.state.websocket_game_response = None,
                        WsAction::Disconnect =>
                            {
                                self.state.websocket_chat_response = None;
                                self.websocket_task.take();
                                self.state.is_connected = false;
                                self.state.is_in_game = false;
                                self.state.piece_color = None;
                            },
                        WsAction::Lost => self.websocket_task = None,
                        WsAction::StartGame => self.state.is_in_game = true,
                        WsAction::ChooseWhiteColor => self.state.piece_color = Some(PieceColor::White),
                        WsAction::ChooseBlackColor => self.state.piece_color = Some(PieceColor::Black),
                        WsAction::LeaveGame =>
                            {
                                self.state.is_in_game = false;
                                self.state.piece_color = None;
                                self.state.websocket_game_response = None;
                            },
                    }
                },
            Msg::Ignore => return false,
            Msg::WsReady(response) =>
                {
                    // if let Some(received_data) = response.map(|data| data).ok()
                    if let Some(received_data) = response.ok()
                    {
                        if let Some(_) = ChatAction::iterator()
                            .position(|action| action.as_str() == received_data.action)
                        {
                            self.state.websocket_chat_response = Some(received_data);
                        }

                        else if let Some(_) = GameAction::iterator()
                            .position(|action| action.as_str() == received_data.action)
                        {
                            self.state.websocket_game_response = Some(received_data);
                        }
                    }
                    else { return false; }
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if !Rc::ptr_eq(&self.props.user, &props.user)
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
        html!
        {
            <main class="main">
                <div class="container">
                    <div class="field">

                        <div class="container">
                            < CheckersChat
                                user=Rc::clone(&self.props.user),
                                is_connected=&self.state.is_connected,
                                disconnect=self.link.callback(|_| Msg::WsAction(WsAction::Disconnect)),
                                connect=self.link.callback(|_| Msg::WsAction(WsAction::Connect)),
                                send_websocket_data=self.link.callback(|request| Msg::WsAction(WsAction::SendWebSocketData(request))),
                                websocket_chat_response=&self.state.websocket_chat_response,
                                reset_websocket_chat_response=self.link.callback(|_| Msg::WsAction(WsAction::ResetWebsocketChatResponse)),
                                is_in_game=&self.state.is_in_game,
                                start_game=self.link.callback(|_| Msg::WsAction(WsAction::StartGame)),
                                choose_white_color=self.link.callback(|_| Msg::WsAction(WsAction::ChooseWhiteColor)),
                                choose_black_color=self.link.callback(|_| Msg::WsAction(WsAction::ChooseBlackColor)),
                             />
                        </div>

                        {
                            html!
                            {
                                <CheckersBoard
                                    user=Rc::clone(&self.props.user),
                                    is_in_game=&self.state.is_in_game,
                                    send_websocket_data=self.link.callback(|request| Msg::WsAction(WsAction::SendWebSocketData(request))),
                                    piece_color=&self.state.piece_color,
                                    websocket_game_response=&self.state.websocket_game_response,
                                    reset_websocket_game_response=self.link.callback(|_| Msg::WsAction(WsAction::ResetWebsocketGameResponse)),
                                    leave_game=self.link.callback(|_| Msg::WsAction(WsAction::LeaveGame)),
                                />
                            }
                        }

                    </div>
                </div>
            </main>
        }
    }
}
