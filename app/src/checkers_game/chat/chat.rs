use std::time::{Duration, Instant};

use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::checkers_game::chat::chat_models::{WsRequest, WsResponse};
use serde_json;


// mod server;

use crate::checkers_game::chat::server;

use crate::DbPool;
use crate::checkers_game::chat::chat_database;


/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


/// Entry point for our route
pub async fn chat_route(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::ChatServer>>, pool: web::Data<DbPool>)
    -> Result<HttpResponse, Error>
{
    ws::start(
        WsChatSession
        {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
            pool
        },
        &req,
        stream,
    )
}


fn insert_new_message(pool: web::Data<DbPool>, name: String, m: String)
{
    let conn = pool.get().expect("couldn't get db connection from pool");
    chat_database::insert_new_message(name.to_owned(), m.to_owned(), &conn);
}


pub async fn extract_chat_log(pool: web::Data<DbPool>, _request: HttpRequest) -> Result<HttpResponse, Error>
{
    let conn = pool.get().expect("couldn't get db connection from pool");
    let all_messages = web::block(move || chat_database::extract_chat_log(&conn))
    .await
    .map_err(|e|
        {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(all_messages))
}


struct WsChatSession
{
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// joined room
    room: String,
    /// peer name
    name: Option<String>,
    /// Chat server
    addr: Addr<server::ChatServer>,
    /// db pool
    pool: web::Data<DbPool>
}

impl Actor for WsChatSession
{
    type Context = ws::WebsocketContext<Self>;


    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context)
    {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect
            {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx|
                {
                    match res
                    {
                        Ok(res) => act.id = res,
                        // something is wrong with chat server
                        _ => ctx.stop(),
                    }
                    fut::ready(())
                })
            .wait(ctx);
    }


    fn stopping(&mut self, _: &mut Self::Context) -> Running
    {
        // notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}


/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for WsChatSession
{
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context)
    {
        ctx.text(msg.0);
    }
}


/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context,)
    {
        let msg = match msg
        {
            Err(_) =>
                {
                    ctx.stop();
                    return;
                },
            Ok(msg) => msg,
        };

        println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg
        {
            ws::Message::Ping(msg) =>
                {
                    self.hb = Instant::now();
                    ctx.pong(&msg);
                },
            ws::Message::Pong(_) =>
                {
                    self.hb = Instant::now();
                },
            ws::Message::Text(text) =>
                {
                    let received_request: Result<WsRequest, _> = serde_json::from_str(&text);
                    if let Ok(request) = received_request
                    {
                        let act = request.action.trim();
                        let m = request.text.trim();

                        match act
                        {
                            "join_to_room" =>
                                {
                                    self.room = m.to_owned();
                                    self.addr.do_send(server::Join
                                    {
                                        id: self.id,
                                        name: self.room.clone(),
                                    });

                                    let response = WsResponse { text: "joined".to_owned() };
                                    ctx.text(serde_json::to_string(&response).unwrap());
                                },
                            "set_name" => self.name = Some(m.to_owned()),
                            "send_message" =>
                                {
                                    let msg = if let Some(ref name) = self.name
                                    {
                                        insert_new_message(self.pool.clone(), name.to_owned(), m.to_owned());
                                        format!("{}: {}", name, m)
                                    }
                                    else
                                    {
                                        m.to_owned()
                                    };
                                    println!("{}", msg);
                                    // send message to chat server
                                    self.addr.do_send(server::ClientMessage
                                    {
                                        id: self.id,
                                        msg,
                                        room: self.room.clone(),
                                    })
                                },
                            _ =>
                                {
                                    let response = WsResponse { text: format!("!!! unknown command: {:?}", m).to_owned() };
                                    ctx.text(serde_json::to_string(&response).unwrap());
                                }
                        }


                        // // we check for /sss type of messages
                        // if m.starts_with('/')
                        // {
                        //     let v: Vec<&str> = m.splitn(2, ' ').collect();
                        //     match v[0]
                        //     {
                        //         "/list" =>
                        //             {
                        //                 // Send ListRooms message to chat server and wait for
                        //                 // response
                        //                 println!("List rooms");
                        //                 self.addr
                        //                     .send(server::ListRooms)
                        //                     .into_actor(self)
                        //                     .then(|res, _, ctx|
                        //                         {
                        //                             match res
                        //                             {
                        //                                 Ok(rooms) =>
                        //                                     {
                        //                                         for room in rooms
                        //                                         {
                        //                                             let response = WsResponse { text: room };
                        //                                             ctx.text(serde_json::to_string(&response).unwrap());
                        //                                         }
                        //                                     }
                        //                                 _ => println!("Something is wrong"),
                        //                             }
                        //                             fut::ready(())
                        //                         })
                        //                     .wait(ctx)
                        //                 // .wait(ctx) pauses all events in context,
                        //                 // so actor wont receive any new messages until it get list
                        //                 // of rooms back
                        //             },
                        //         "/join" =>
                        //             {
                        //                 if v.len() == 2
                        //                 {
                        //                     self.room = v[1].to_owned();
                        //                     self.addr.do_send(server::Join
                        //                     {
                        //                         id: self.id,
                        //                         name: self.room.clone(),
                        //                     });
                        //
                        //                     let response = WsResponse { text: "joined".to_owned() };
                        //                     ctx.text(serde_json::to_string(&response).unwrap());
                        //                 }
                        //                 else
                        //                 {
                        //                     let response = WsResponse { text: "!!! room name is required".to_owned() };
                        //                     ctx.text(serde_json::to_string(&response).unwrap());
                        //                 }
                        //             },
                        //         "/name" =>
                        //             {
                        //                 if v.len() == 2
                        //                 {
                        //                     self.name = Some(v[1].to_owned());
                        //                 }
                        //                 else
                        //                 {
                        //                     let response = WsResponse { text: "!!! name is required".to_owned() };
                        //                     ctx.text(serde_json::to_string(&response).unwrap());
                        //                 }
                        //             },
                        //         _ =>
                        //             {
                        //                 let response = WsResponse { text: format!("!!! unknown command: {:?}", m).to_owned() };
                        //                 ctx.text(serde_json::to_string(&response).unwrap());
                        //             },
                        //     }
                        // }
                        // else
                        // {
                        //     let msg = if let Some(ref name) = self.name
                        //     {
                        //         format!("{}: {}", name, m)
                        //     }
                        //     else
                        //     {
                        //         m.to_owned()
                        //     };
                        //     println!("{}", msg);
                        //     // send message to chat server
                        //     self.addr.do_send(server::ClientMessage
                        //     {
                        //         id: self.id,
                        //         msg,
                        //         room: self.room.clone(),
                        //     })
                        // }

                    }
                },
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) =>
                {
                    ctx.close(reason);
                    ctx.stop();
                },
            ws::Message::Continuation(_) =>
                {
                    ctx.stop();
                },
            ws::Message::Nop => (),
        }
    }
}


impl WsChatSession
{
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>)
    {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx|
            {
                // check client heartbeats
                if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT
                {
                    // heartbeat timed out
                    println!("Websocket Client heartbeat failed, disconnecting!");

                    // notify chat server
                    act.addr.do_send(server::Disconnect { id: act.id });

                    // stop actor
                    ctx.stop();

                    // don't try to send a ping
                    return;
                }

                ctx.ping(b"");
            });
    }
}
