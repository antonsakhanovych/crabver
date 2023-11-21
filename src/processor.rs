use crate::message::MessageType;

use std::io::Write;
use std::net::TcpStream;
use std::sync::{mpsc::Receiver, Arc};

pub struct Processor {
    incoming: Receiver<MessageType>,
    clients: Vec<Arc<TcpStream>>,
}

impl Processor {
    pub fn new(incoming: Receiver<MessageType>) -> Self {
        Self {
            incoming,
            clients: Vec::new(),
        }
    }

    // Consumes itself
    pub fn serve(mut self) -> () {
        while let Ok(message) = self.incoming.recv() {
            match message {
                MessageType::ClientConnected(client) => self.clients.push(client),
                MessageType::Text(client, name, message) => {
                    self.send_out_message(client, name.as_ref(), message.trim())
                }
                MessageType::ClientDisconnected(client) => {
                    self.clients.retain(|c| !Arc::ptr_eq(&client, &c))
                }
            }
        }
        ()
    }

    fn send_out_message(&self, client: Arc<TcpStream>, name: &str, message: &str) {
        for stream in &self.clients {
            if Arc::ptr_eq(stream, &client) {
                continue;
            }
            let mut stream = stream.as_ref();
            print!("{}", name);
            let packet = [name.trim(), ": ", message, "\n"].join("");
            let _ = stream.write(packet.as_bytes());
        }
    }
}
