//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};

use crate::checkers_game::chat::chat_models::WsResponse;
use serde_json;


/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect
{
    pub addr: Recipient<Message>,
}


/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect
{
    pub id: usize,
}


/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage
{
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}


/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join
{
    /// Client id
    pub id: usize,
    /// Room name
    pub name: String,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct SetUserName
{
    /// Client id
    pub id: usize,
    /// User name
    pub user_name: String,
}


#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct ListUserNames
{
    /// Id of the client session
    pub id: usize,
    /// Room name
    pub room: String,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct Invitation
{
    pub id: usize,
    pub to_user: String,
    pub room: String,
    pub action: String,
}


/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct GameMessage
{
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}



#[derive(Clone)]
struct SessionData
{
    recipient: Recipient<Message>,
    user_name: Option<String>,
}


/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChatServer
{
    sessions: HashMap<usize, SessionData>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
}


impl Default for ChatServer
{
    fn default() -> ChatServer
    {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());

        ChatServer
        {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
        }
    }
}


impl ChatServer
{
    /// Send message to all users in the room
    fn send_message(&self, room: &str, action: &str, message: &str, skip_id: usize)
    {
        if let Some(sessions) = self.rooms.get(room)
        {
            for id in sessions
            {
                if *id != skip_id
                {
                    if let Some(addr) = self.sessions.get(id)
                    {
                        let response = WsResponse { action: action.to_owned(), data: message.to_owned() };
                        let m = serde_json::to_string(&response).unwrap();
                        let _ = addr.recipient.do_send(Message(m));
                    }
                }
            }
        }
    }


    fn process_invitation(&self, room: &str, from_user: &str, to_user: &str, action: &str)
    {
        if let Some(sessions) = self.rooms.get(room)
        {
            for id in sessions
            {
                if let Some(addr) = self.sessions.get(id)
                {
                    if let Some(user_name) = &addr.user_name
                    {
                        if user_name == to_user
                        {
                            let response = WsResponse { action: action.to_owned(), data: from_user.to_owned() };
                            let m = serde_json::to_string(&response).unwrap();
                            let _ = addr.recipient.do_send(Message(m));
                        }
                    }
                }
            }
        }
    }
}


/// Make actor from `ChatServer`
impl Actor for ChatServer
{
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}


/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer
{
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result
    {
        println!("Someone joined");

        // notify all users in same room
        self.send_message(&"Main".to_owned(), "connect", "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(
            id,
            SessionData { recipient: msg.addr, user_name: None }
        );


        // auto join session to Main room
        self.rooms
            .entry("Main".to_owned())
            .or_insert(HashSet::new())
            .insert(id);

        // send id back
        id
    }
}


/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer
{
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>)
    {
        let mut rooms: Vec<String> = Vec::new();

        // remove address
        // if self.sessions.remove(&msg.id).is_some()
        // remove adress and extract user_name
        let disconnected_user_name =
            {
                if let Some(session_data) = self.sessions.remove(&msg.id)
                {
                    // remove session from all rooms
                    for (name, sessions) in &mut self.rooms
                    {
                        if sessions.remove(&msg.id)
                        {
                            rooms.push(name.to_owned());
                        }
                    }
                    session_data.user_name
                }
                else
                {
                    None
                }
            };

        match disconnected_user_name
        {
            Some(user_name) =>
                {
                    // send message to other users
                    for room in rooms
                    {
                        self.send_message(&room, "disconnect", &user_name, 0);
                    }
                    println!("'{}' disconnected", user_name);
                }
            None =>
                {
                    // send message to other users
                    for room in rooms
                    {
                        self.send_message(&room, "disconnect", "Someone disconnected", 0);
                    }
                    println!("Someone disconnected");
                }
        }
    }
}


/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer
{
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>)
    {
        self.send_message(&msg.room, "received_message", msg.msg.as_str(),  msg.id);
    }
}


/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChatServer
{
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>)
    {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms
        {
            if sessions.remove(&id)
            {
                rooms.push(n.to_owned());
            }
        }

        let user_name =
            {
                if let Some(session_data) = self.sessions.get(&id)
                {
                    session_data.user_name.clone()
                }
                else
                {
                    None
                }
            };

        match user_name
        {
            Some(user_name) =>
                {
                    // send message to other users
                    for room in rooms
                    {
                        self.send_message(&room, "disconnect", &user_name, 0);
                    }
                },
            None =>
                {
                    // send message to other users
                    for room in rooms
                    {
                        self.send_message(&room, "disconnect", "Someone disconnected", 0);
                    }
                }
        }

        self.rooms
            .entry(name.clone())
            .or_insert(HashSet::new())
            .insert(id);

        self.send_message(&name, "connect", "Someone connected", id);
    }
}


impl Handler<SetUserName> for ChatServer
{
    type Result = ();

    fn handle(&mut self, msg: SetUserName, _: &mut Context<Self>)
    {
        if let Some(session) = self.sessions.clone().get(&msg.id)
        {
            self.sessions.insert(
                msg.id,
                SessionData { recipient: session.recipient.clone(), user_name: Some(msg.user_name.to_owned()) }
            );
        }
    }
}


impl Handler<ListUserNames> for ChatServer
{
    type Result = MessageResult<ListUserNames>;

    fn handle(&mut self, msg: ListUserNames, _: &mut Context<Self>) -> Self::Result
    {
        let mut user_names = Vec::new();

        if let Some(required_sessions) = self.rooms.get(&msg.room)
        {
            for id in required_sessions
            {
                if *id != msg.id
                {
                    if let Some(session) = self.sessions.get(&id)
                    {
                        if let Some(user_name) = &session.user_name
                        {
                            user_names.push(user_name.to_owned())
                        }
                    }
                }
            }
        }
        MessageResult(user_names)
    }
}


impl Handler<Invitation> for ChatServer
{
    type Result = ();

    fn handle(&mut self, msg: Invitation, _: &mut Context<Self>)
    {
        if let Some(session) = self.sessions.clone().get(&msg.id)
        {
            if let Some(user_name) = &session.user_name
            {
                self.process_invitation(&msg.room, &user_name, &msg.to_user, &msg.action);
            }
        }
    }
}


/// Handler for Game message.
impl Handler<GameMessage> for ChatServer
{
    type Result = ();

    fn handle(&mut self, msg: GameMessage, _: &mut Context<Self>)
    {
        self.send_message(&msg.room, "received_checker_piece_move", msg.msg.as_str(),  msg.id);
    }
}
