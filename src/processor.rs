use crate::{client::Client, message::MessageType};

use std::sync::mpsc::Receiver;

pub struct Processor {
    incoming: Receiver<MessageType>,
    clients: Vec<Client>,
}

impl Processor {
    pub fn new(incoming: Receiver<MessageType>) -> Self {
        Self {
            incoming,
            clients: Vec::new(),
        }
    }

    // Consumes itself
    pub fn serve(self) -> ! {
        loop {}
    }
}
