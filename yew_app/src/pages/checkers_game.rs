use yew::prelude::*;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use web_sys;


use crate::types::{AuthorizedUserResponse};


#[derive(Properties, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
}


#[derive(Serialize, Debug)]
struct WsRequest
{
    action: String,
    text: String,
}


#[derive(Deserialize, Debug)]
pub struct WsResponse
{
    text: String,
}


struct ChatMessage
{
    message: String
}


struct State
{
    message: Option<String>,
    chat_messages: Vec<ChatMessage>,
    is_connected: bool,
    is_chat_room_defined: bool,
}


pub struct CheckersGame
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    ws: Option<WebSocketTask>,
}


impl CheckersGame
{
    fn add_message_to_content(&mut self, message: String)
    {
        self.state.chat_messages.push(ChatMessage { message });
    }
}


pub enum WsAction
{
    Connect,
    SendData,
    Disconnect,
    Lost,
}


impl From<WsAction> for Msg
{
    fn from(action: WsAction) -> Self
    {
        Msg::WsAction(action)
    }
}


pub enum Msg
{
    UpdateEditMessage(String),
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    // WsReady(Result<String, Error>),
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
            state: State { message: None, chat_messages: Vec::new(), is_connected: false, is_chat_room_defined: false },
            ws: None
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        if let Some(_) = &self.ws
        {
            if !self.state.is_chat_room_defined
            {
                let join_to_room_request = WsRequest { action: "join_to_room".to_owned(), text: "/join checkers_game".to_owned() };
                self.ws.as_mut().unwrap().send(Json(&join_to_room_request));

                if let Some(user) = &self.props.user
                {
                    let set_name_request = WsRequest { action: "set_name".to_owned(), text: format!("/name {}", user.user_name) };
                    self.ws.as_mut().unwrap().send(Json(&set_name_request));
                }
                self.state.is_chat_room_defined = true;
            }
        }

        match msg
        {
            Msg::UpdateEditMessage(e) => self.state.message = Some(e),
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
                                    WebSocketService::connect("ws://0.0.0.0:8080/ws/", callback, notification)
                                        .unwrap();
                                self.ws = Some(task);
                                self.state.is_connected = true;
                                self.state.is_chat_room_defined = false;
                            },
                        WsAction::SendData =>
                            {
                                if let Some(message) = &self.state.message.clone()
                                {
                                    if !message.is_empty()
                                    {
                                        if let Some(_) = &self.ws
                                        {
                                            self.add_message_to_content(message.to_owned());

                                            // self.ws.as_mut().unwrap().send(Json(&message));

                                            let request = WsRequest { action: "send_message".to_owned(), text: message.to_owned() };
                                            self.ws.as_mut().unwrap().send(Json(&request));

                                            self.state.message = None;
                                        }
                                        else { return false; }
                                    }
                                    else { return false; }
                                }
                                else { return false; }
                            },
                        WsAction::Disconnect =>
                            {
                                self.ws.take();
                                self.state.is_connected = false;
                            },
                        WsAction::Lost => { self.ws = None; },
                    }
                },
            Msg::Ignore => { return false; },
            Msg::WsReady(response) =>
                {
                    if let Some(received_data) = response.map(|data| data.text).ok()
                    {
                        self.add_message_to_content(received_data);
                    }
                    else { return false; }
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
                                html! { <> { &chat_message.message } <br /> </>  }
                            })
                        }
                    </div>
                     <form onSubmit="return false;">
                        <input
                            oninput=self.link.callback(|e: InputData| Msg::UpdateEditMessage(e.value)) />
                        {
                            if let Some(_) = &self.props.user
                            {
                                if self.state.is_connected
                                {
                                    html! { <button type="reset" onclick=self.link.callback(|_| WsAction::SendData)>{ "Send" }</button> }
                                }
                                else
                                {
                                    html! { <button disabled=true>{ "Send" }</button> }
                                }
                            }
                            else
                            {
                                html! { <button disabled=true>{ "Send" }</button> }
                            }
                        }
                    </form>
                </div>
            </main>
        }
    }


    fn rendered(&mut self, first_render: bool)
    {
        if first_render
        {
            self.link.send_message(WsAction::Connect);
        }

        if let Some(element) = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("checkers_game_chat_log")
        {
            element.set_scroll_top(element.scroll_height());
        }
    }
}
