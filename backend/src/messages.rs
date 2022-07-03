use actix::prelude::*;

use crate::{client, room};

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct CreateRoomMessage {
    pub client_id: u32,
    pub client_addr: Addr<client::Client>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct JoinRoomMessage {
    pub client_id: u32,
    pub client_addr: Addr<client::Client>,
    pub room_id: u32,
}

#[derive(Message, Debug)]
#[rtype(u32)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub inbox_addr: Addr<client::Client>,
    pub color: String,
}

#[derive(Message, Debug, Clone)]
#[rtype(u32)]
pub struct Disconnect {
    pub id: u32,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct RemoveFromRoom {
    pub client_id: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct TileMessage {
    /// Id of the client session
    pub id: u32,
    /// Peer message
    pub msg: room::Tile,
    pub room_id: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectClientToRoom {
    pub room_id: u32,
}
