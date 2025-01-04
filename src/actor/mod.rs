use actix::{Actor, Addr, AsyncContext, Handler, Message, Running, StreamHandler};
use actix_web_actors::ws;
use crate::ws_sess_manager::WsSessionManager;

// Message Structs for WebSocket Events

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub(crate) addr: Addr<WebSocket>,

}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub(crate) addr: Addr<WebSocket>,

}


// a message to be broadcast to all connected WS clients
// msg = the contect that will be sent
/// sender = address of the WS actor that sent the msg
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
  pub(crate) msg: String,
  pub(crate) sender: Addr<WebSocket>

}