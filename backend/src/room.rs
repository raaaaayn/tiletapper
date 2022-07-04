use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::messages::{JoinRoomMessage, Message};

#[derive(Deserialize, Serialize)]
pub struct Tile {
    pub tile_num: u16,
    pub color: String,
}

pub struct Room {
    pub clients: HashMap<u32, Recipient<Message>>,
    pub board: Vec<String>,
}

impl Room {
    pub fn new() -> Room {
        let board: Vec<String> = vec!["#FFFFFF".to_owned(); 20];
        Room {
            clients: HashMap::default(),
            board,
        }
    }
    pub fn add_client(&mut self, msg: &JoinRoomMessage) {
        self.clients
            .insert(msg.client_id, msg.client_addr.clone().recipient());
    }
    pub fn remove_client(&mut self, client_id: u32) -> u32 {
        self.clients.remove(&client_id);
        client_id
    }
    pub fn send_message_to_all_clients(&self, msg: String) {
        for (_id, addr) in self.clients.iter() {
            addr.do_send(Message(msg.clone()));
        }
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}
