use actix::prelude::*;
use actix_web_actors::ws;
use rand::prelude::*;
use random_color;
use serde::{Deserialize, Serialize};

use crate::messages::{
    Connect, ConnectClientToRoom, CreateRoomMessage, Disconnect, JoinRoomMessage, Message,
    RemoveFromRoom, TileMessage,
};
use crate::room;
use crate::server;

#[derive(Serialize, Deserialize, Debug)]
pub enum GameMessageType {
    Create,
    Join,
    Tile,
    Exit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMessage {
    pub r#type: GameMessageType,
    pub data: Option<u32>,
}

#[derive(Debug)]
pub struct Client {
    pub id: u32,
    pub server: Addr<server::Server>,
    pub room_id: Option<u32>,
    pub color: String,
}

#[derive(Deserialize, Serialize)]
struct BoardData {
    tile_num: u16,
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.server
            .send(Connect {
                addr: addr.clone().recipient(),
                inbox_addr: addr,
                color: self.color.to_owned(),
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
        self.server.do_send(Disconnect { id: self.id });
        // if let Some(room) = self.room.as_ref() {
        //     room.do_send(Disconnect { id: self.id });
        // }
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = msg.unwrap();
        match msg {
            ws::Message::Text(text) => {
                // print!("got msg ");
                let msg = text.trim().to_string();
                if let Ok(game_msg) = serde_json::from_str::<GameMessage>(&msg) {
                    // print!("game_msg_yes ");
                    match game_msg.r#type {
                        GameMessageType::Join => {
                            if let Some(id) = game_msg.data {
                                self.server.do_send(JoinRoomMessage {
                                    client_id: self.id,
                                    client_addr: ctx.address(),
                                    room_id: id,
                                })
                            }
                        }
                        GameMessageType::Create => self.server.do_send(CreateRoomMessage {
                            client_id: self.id,
                            client_addr: ctx.address(),
                        }),
                        GameMessageType::Tile => {
                            // print!("Tile ");
                            if let Some(room_id) = self.room_id {
                                // print!("room_id_yes ");
                                if let Some(tile_num) = game_msg.data {
                                    // print!("tile_num_yes");
                                    let tile = room::Tile {
                                        tile_num: tile_num as u16,
                                        color: self.color.to_owned(),
                                    };
                                    self.server.do_send(TileMessage {
                                        id: self.id,
                                        msg: tile,
                                        room_id,
                                    })
                                };
                            }
                        }
                        GameMessageType::Exit => {
                            self.room_id = None;
                            self.server.do_send(RemoveFromRoom { client_id: self.id });
                        }
                    }
                }
                // println!();
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

fn gen_id() -> u32 {
    let mut th = thread_rng();
    let id: u32 = th.gen();
    id
}

impl Client {
    pub fn new(server_addr: Addr<server::Server>, room_id: Option<u32>) -> Client {
        Client {
            id: gen_id(),
            room_id,
            server: server_addr,
            color: random_color::RandomColor::new().to_hex(),
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
    fn handle(&mut self, msg: ConnectClientToRoom, _ctx: &mut Self::Context) {
        self.room_id = Some(msg.room_id);
    }
}
