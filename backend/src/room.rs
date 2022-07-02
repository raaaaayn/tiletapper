use actix::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

use crate::{
    client,
    messages::{ClientMessage, Disconnect, JoinRoomMessage, Message},
};

#[derive(Deserialize, Serialize)]
pub struct Tile {
    pub tile_num: u16,
    pub color: String,
}

pub struct Room {
    pub clients: HashMap<u32, Recipient<Message>>,
    pub board: Vec<String>,
}

impl Actor for Room {
    type Context = Context<Self>;
}

impl Room {
    pub fn new() -> Room {
        let mut board: Vec<String> = Vec::new();
        for _ in 0..21 {
            board.push("#FFFFFF".to_owned());
        }
        Room {
            clients: HashMap::default(),
            board,
        }
    }
    fn add_client(&mut self, msg: JoinRoomMessage) {
        self.clients
            .insert(msg.client_id, msg.client_addr.recipient());
    }
    fn remove_client(&mut self, client_id: u32) -> u32 {
        self.clients.remove(&client_id);
        client_id
    }
    fn send_message_to_all_clients(&self, msg: String) {
        for (_id, addr) in self.clients.iter() {
            addr.do_send(Message(msg.clone()));
        }
    }
    fn send_message_to_self(&self, client_addr: Addr<client::Client>, msg: String) {
        client_addr.do_send(Message(msg));
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

impl Handler<ClientMessage> for Room {
    type Result = ();
    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        let tile = self.board.get_mut(msg.msg.tile_num as usize); // cannot index into vec with u16
                                                                  // because SliceIndex trait is
                                                                  // only implemented for usize

        if let Some(color) = tile {
            *color = msg.msg.color.clone()
        }

        let stringified = serde_json::to_string(&msg.msg);
        match stringified {
            Ok(msg) => {
                let _ = self.send_message_to_all_clients(format!("tile\n{}", msg));
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    }
}

impl Handler<JoinRoomMessage> for Room {
    type Result = ();
    fn handle(&mut self, msg: JoinRoomMessage, _: &mut Context<Self>) {
        // // converting board hasmap to vector
        // let mut constructed_board: Vec<Tile> = self
        //     .board
        //     .iter()
        //     .map(|(tile_num, color)| Tile {
        //         tile_num: *tile_num,
        //         color: color.clone(),
        //     })
        //     .collect();
        // constructed_board.sort_by_key(|tile| tile.tile_num);
        self.send_message_to_self(
            msg.client_addr.clone(),
            format!("board\n{}", serde_json::to_string(&self.board).unwrap()),
        );
        self.add_client(msg);
    }
}

impl Handler<Disconnect> for Room {
    type Result = u32;
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        self.remove_client(msg.id)
    }
}
