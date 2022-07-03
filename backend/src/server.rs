use std::collections::HashMap;

use crate::{
    client::Client,
    messages::{
        Connect, ConnectClientToRoom, CreateRoomMessage, Disconnect, JoinRoomMessage, Message,
        RemoveFromRoom, TileMessage,
    },
    room::Room,
};

use actix::prelude::*;
use rand::prelude::*;

pub struct Server {
    pub rooms: HashMap<u32, Room>,
    pub clients: HashMap<u32, Addr<Client>>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Server {
    pub fn new() -> Server {
        Server {
            rooms: HashMap::default(),
            clients: HashMap::default(),
        }
    }

    fn create_room(&mut self, msg: CreateRoomMessage) -> u32 {
        let mut th = thread_rng();
        let id: u32 = th.gen();
        let mut room = Room::new();

        room.add_client(&JoinRoomMessage {
            client_id: msg.client_id,
            client_addr: msg.client_addr.clone(),
            room_id: id,
        });

        self.send_message_through_client(
            &msg.client_id,
            format!("board\n{}", serde_json::to_string(&room.board).unwrap()),
        );
        msg.client_addr.do_send(ConnectClientToRoom { room_id: id });

        self.rooms.insert(id, room);
        self.send_message_through_all_clients(format!("Room\n{}", id.to_string()));
        id
    }

    fn join_room(&mut self, msg: JoinRoomMessage) {
        // println!("JoinRoomMessage");
        let res = self.rooms.get_mut(&msg.room_id);
        match res {
            Some(room) => {
                room.add_client(&msg);
                msg.client_addr.do_send(ConnectClientToRoom {
                    room_id: msg.room_id,
                });
            }
            None => {
                eprintln!("Room does not exist\nmsg room id {}", msg.room_id)
            }
        }
        if let Some(room) = self.rooms.get(&msg.room_id) {
            // println!("got immut");
            self.send_message_through_client(
                &msg.client_id,
                format!("board\n{}", serde_json::to_string(&room.board).unwrap()),
            );
        }
    }
    fn _delete_room(&mut self, id: u32) -> u32 {
        self.rooms.remove(&id);
        id
    }
    fn add_client(&mut self, addr: Addr<Client>) -> u32 {
        let mut th = thread_rng();
        let id: u32 = th.gen();
        self.clients.insert(id, addr);
        id
    }
    fn remove_client(&mut self, id: u32) -> u32 {
        self.clients.remove(&id);
        id
    }
    fn send_message_through_client(&self, client_id: &u32, msg: String) {
        if let Some(client_addr) = self.clients.get(client_id) {
            client_addr.do_send(Message(msg))
        }
    }
    fn send_message_through_all_clients(&self, msg: String) {
        for (_id, addr) in self.clients.iter() {
            addr.do_send(Message(msg.clone()));
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Handler<Connect> for Server {
    type Result = u32;
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let client_id = self.add_client(msg.inbox_addr.clone());
        if !self.rooms.is_empty() {
            let mut rooms_str = String::from("rooms");
            for key in self.rooms.keys().into_iter() {
                rooms_str += &format!("\n{}", key);
            }
            msg.addr.do_send(Message(rooms_str));
        }
        msg.addr.do_send(Message(format!("color\n{}", msg.color)));
        // self.join_room(JoinRoomMessage {
        //     client_id: client_id.clone(),
        //     client_addr: msg.inbox_addr,
        // });
        client_id
    }
}

impl Handler<CreateRoomMessage> for Server {
    type Result = ();
    fn handle(&mut self, msg: CreateRoomMessage, _: &mut Context<Self>) -> Self::Result {
        self.create_room(msg);
    }
}

impl Handler<JoinRoomMessage> for Server {
    type Result = ();
    fn handle(&mut self, msg: JoinRoomMessage, _: &mut Context<Self>) -> Self::Result {
        self.join_room(msg);
    }
}

impl Handler<Disconnect> for Server {
    type Result = u32;
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        for (_id, room) in self.rooms.iter_mut() {
            room.remove_client(msg.id);
        }
        self.remove_client(msg.id)
    }
}

impl Handler<RemoveFromRoom> for Server {
    type Result = ();
    fn handle(&mut self, msg: RemoveFromRoom, _: &mut Context<Self>) -> Self::Result {
        for (_id, room) in self.rooms.iter_mut() {
            room.remove_client(msg.client_id);
        }
    }
}

impl Handler<TileMessage> for Server {
    type Result = ();
    fn handle(&mut self, msg: TileMessage, _: &mut Context<Self>) {
        // print!("Room ");
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            let tile = room.board.get_mut(msg.msg.tile_num as usize); // cannot index into vec with u16
                                                                      // because SliceIndex trait is
                                                                      // only implemented for usize

            if let Some(color) = tile {
                *color = msg.msg.color.clone()
            }

            let stringified = serde_json::to_string(&msg.msg);
            match stringified {
                Ok(msg) => {
                    let _ = room.send_message_to_all_clients(format!("tile\n{}", msg));
                    // println!("msg");
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        }
    }
}
