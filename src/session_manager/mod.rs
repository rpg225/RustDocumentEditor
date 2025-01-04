use std::collections::HashSet;
use actix::{Actor, Addr, Context, Handler};
use crate::ws_sess_manager::WsSessionManager;

pub struct WsSessionManager {

    /// A set set to track the addresses of connected WebSocket actors
    sessions: HashSet<Addr<WebSocket>>,

    // Stores the last text message broadcast to clients
    pub(crate) last_text: String,
}

impl WsSessionManager {

}