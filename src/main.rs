use crabver::{client::Client, message::MessageType, processor::Processor};
use std::net::{SocketAddr, TcpListener};
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc,
};

use std::thread;

fn main() {
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(address).unwrap();

    println!("Listenting on: {}", address);

    let (sender, receiver): (Sender<MessageType>, Receiver<MessageType>) = mpsc::channel();

    let processor = Processor::new(receiver);
    thread::spawn(move || processor.serve());
    for stream in listener.incoming() {
        let sender = sender.clone();
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                let client = Client::new(stream, sender);
                thread::spawn(move || client.handle());
            }
            Err(e) => eprintln!("Couldn't accept because of {}", e),
        }
    }
}
