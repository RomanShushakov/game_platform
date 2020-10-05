use yew::prelude::*;
use web_sys;

use crate::types::{AuthorizedUserResponse, WsRequest};
use crate::pages::{ChatMessage, Actions};


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
    pub chat_messages: Vec<ChatMessage>,
    pub is_connected: bool,
    pub disconnect: Callback<()>,
    pub connect: Callback<()>,
    pub send_message: Callback<WsRequest>
}



struct State
{
    message: Option<String>,
    // online_users: HashSet<OnlineUser>,
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
}


pub enum Msg
{
    Disconnect,
    Connect,
    UpdateEditMessage(String),
    DefineButton(u32),
    SendMessage,
}


impl Component for CheckersChat
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, state: State { message: None } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::Disconnect => self.props.disconnect.emit(()),
            Msg::Connect => self.props.connect.emit(()),
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
                            self.props.send_message.emit(
                                WsRequest {
                                    action: Actions::SendMessage.as_str(),
                                    data: format!("you: {}", message)
                                });
                            self.state.message = None;
                        }
                        else { return false; }
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
                        for self.props.chat_messages.iter().map(|chat_message: &ChatMessage|
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

            //     <h3>{ "Users online" }</h3>
            //     <div class="checkers_game_online_users">
            //         <table>
            //             // <thead>
            //             //     <tr>
            //             //         <th>{ "User name" }</th>
            //             //     </tr>
            //             // </thead>
            //             <tbody>
            //             {
            //                 for self.state.online_users.iter().map(|online_user: &OnlineUser|
            //                 html!
            //                 {
            //                     <tr>
            //                         <td>{ &online_user.0 }</td>
            //                         <td>
            //                             {
            //                                 if true
            //                                 {
            //                                     let user_name = online_user.0.clone();
            //                                     html!
            //                                     {
            //                                         <button
            //                                             onclick=self.link.callback(move |_| WsAction::SendInvitation(user_name.clone()))
            //                                             disabled=self.invitation_status_check(&user_name)>
            //                                             { "invite to play" }
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
}