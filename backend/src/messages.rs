use actix::prelude::*;

use crate::{client, room};

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct CreateRoomMessage {
    pub client_id: usize,
    pub client_addr: Addr<client::Client>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct JoinRoomMessage {
    pub client_id: usize,
    pub client_addr: Addr<client::Client>,
    pub room_id: usize,
}

#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub inbox_addr: Addr<client::Client>,
}

#[derive(Message, Debug, Clone)]
#[rtype(usize)]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: room::Tile,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectClientToRoom {
    pub room_addr: Addr<room::Room>,
}
