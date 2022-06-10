use actix::prelude::*;
use actix_web_actors::ws;
use rand::prelude::*;
use random_color;
use serde::{Deserialize, Serialize};

use crate::messages::{
    ClientMessage, Connect, ConnectClientToRoom, CreateRoomMessage, Disconnect, JoinRoomMessage,
    Message,
};
use crate::room::{self, Room};
use crate::server;

#[derive(Debug)]
pub struct Client {
    pub id: usize,
    pub room: Option<Addr<Room>>,
    pub server: Addr<server::Server>,
    pub color: String,
}

#[derive(Deserialize, Serialize)]
struct BoardData {
    tile_num: u16,
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("New websocket connection");
        let addr = ctx.address().clone();
        self.server
            .send(Connect {
                addr: addr.clone().recipient(),
                inbox_addr: addr,
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Socket closed");
        self.server.do_send(Disconnect { id: self.id });
        match self.room.as_ref() {
            Some(room) => {
                room.do_send(Disconnect { id: self.id });
            }
            None => {}
        }
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = msg.unwrap();
        match msg {
            ws::Message::Text(text) => {
                println!("Got message");
                let msg = format!("{}", text.trim());
                if msg.starts_with("create") {
                    self.server.do_send(CreateRoomMessage {
                        client_id: self.id,
                        client_addr: ctx.address().clone(),
                    })
                } else if msg.starts_with("join") {
                    let raw_str: Vec<&str> = msg.rsplitn(2, "\n").collect();
                    match raw_str.get(0) {
                        Some(str_id) => {
                            let id = str_id.parse::<usize>();
                            match id {
                                Ok(id) => {
                                    println!("{}", id);
                                    self.server.do_send(JoinRoomMessage {
                                        client_id: self.id,
                                        client_addr: ctx.address().clone(),
                                        room_id: id,
                                    })
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            }
                        }
                        None => {}
                    }
                } else if msg.starts_with("exit") {
                    self.room = None;
                    self.server.do_send(Disconnect { id: self.id });
                } else {
                    let board_data = serde_json::from_str(&msg).unwrap_or(BoardData {tile_num:0});
                    let tile = room::Tile {
                        tile_num: board_data.tile_num,
                        color: Some(self.color.to_owned()),
                    };
                    match self.room.as_ref() {
                        Some(room) => {
                            room.do_send(ClientMessage {
                                id: self.id,
                                msg: tile,
                            });
                        }
                        None => {}
                    }
                }
            }
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Ping(bytes) => ctx.pong(&bytes),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

fn gen_id() -> usize {
    let mut th = thread_rng();
    let id: usize = th.gen();
    id
}

impl Client {
    pub fn new(server_addr: Addr<server::Server>, room_addr: Option<Addr<Room>>) -> Client {
        Client {
            id: gen_id(),
            room: room_addr,
            server: server_addr,
            color: random_color::RandomColor::new().to_rgb_string(),
        }
    }
}

impl Handler<Message> for Client {
    type Result = ();
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Handler<ConnectClientToRoom> for Client {
    type Result = ();
    fn handle(&mut self, msg: ConnectClientToRoom, ctx: &mut Self::Context) {
        self.room = Some(msg.room_addr);
    }
}
