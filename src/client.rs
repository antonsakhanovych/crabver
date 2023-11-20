use crate::message::MessageType;
use std::net::TcpStream;
use std::sync::{mpsc::Sender, Arc};

pub struct Client {
    stream: Arc<TcpStream>,
    sender: Sender<MessageType>,
}

impl Client {
    pub fn new(stream: Arc<TcpStream>, sender: Sender<MessageType>) -> Self {
        Self { stream, sender }
    }
}

pub fn handle_client(client: Client) {}
