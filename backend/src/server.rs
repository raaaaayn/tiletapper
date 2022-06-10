use std::collections::HashMap;

use crate::{
    client::Client,
    messages::{
        ClientMessage, Connect, ConnectClientToRoom, CreateRoomMessage, Disconnect,
        JoinRoomMessage, Message,
    },
    room::Room,
};

use actix::prelude::*;
use rand::prelude::*;

struct After {
    count: i32,
}

pub struct Server {
    pub rooms: HashMap<usize, Addr<Room>>,
    pub clients: HashMap<usize, Addr<Client>>,
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

    fn create_room(&mut self, msg: CreateRoomMessage) -> Addr<Room> {
        let mut th = thread_rng();
        let id: usize = th.gen();
        println!("Room {} added", id);
        let room = Room::new().start();
        room.do_send(JoinRoomMessage {
            client_id: msg.client_id,
            client_addr: msg.client_addr,
            room_id: id,
        });
        self.rooms.insert(id, room.clone());
        room.clone()
    }

    fn join_room(&mut self, msg: JoinRoomMessage) {
        let res = self.rooms.get(&msg.room_id);
        match res {
            Some(room) => {
                msg.client_addr.do_send(ConnectClientToRoom {
                    room_addr: room.clone(),
                });
                room.do_send(JoinRoomMessage {
                    client_id: msg.client_id,
                    client_addr: msg.client_addr,
                    room_id: msg.room_id,
                })
            }
            None => {
                eprintln!("Room does not exist")
            }
        }
    }
    fn delete_room(&mut self, id: usize) -> usize {
        self.rooms.remove(&id);
        println!("\n{:#?}\n", self.rooms);
        id
    }
    fn add_client(&mut self, addr: Addr<Client>) -> usize {
        let mut th = thread_rng();
        let id: usize = th.gen();
        println!("Client {} added", id);
        self.clients.insert(id, addr);
        id
    }
    fn remove_client(&mut self, id: usize) -> usize {
        self.clients.remove(&id);
        println!("\n{:#?}\n", self.clients);
        id
    }
}

impl Handler<Connect> for Server {
    type Result = usize;
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined {:?}", msg);
        let client_id = self.add_client(msg.inbox_addr.clone());
        if !self.rooms.is_empty() {
            let mut rooms_str = String::from("rooms");
            for key in self.rooms.keys().into_iter() {
                rooms_str += &format!("\n{}", key);
            }
            msg.addr.do_send(Message(rooms_str));
        }
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
    type Result = usize;
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone left {:?}", msg);
        for (_id, room) in self.rooms.iter() {
            room.do_send(msg.to_owned())
        }
        self.remove_client(msg.id)
    }
}

impl Handler<ClientMessage> for Server {
    type Result = ();
    fn handle(&mut self, _msg: ClientMessage, _: &mut Context<Self>) {}
}
