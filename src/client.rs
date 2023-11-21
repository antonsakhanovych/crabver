use crate::message::MessageType;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::str;
use std::sync::{mpsc::Sender, Arc};

pub struct Client {
    name: Arc<String>,
    stream: Arc<TcpStream>,
    sender: Sender<MessageType>,
}

impl Client {
    pub fn new(stream: Arc<TcpStream>, sender: Sender<MessageType>) -> Self {
        Self {
            name: Arc::new(String::new()),
            stream,
            sender,
        }
    }

    fn get_name(&mut self) {
        let mut buf = [0; 255];
        let mut stream = self.stream.as_ref();
        let _ = stream.write("Enter your name: ".as_bytes());
        let read_size = stream.read(&mut buf).unwrap();

        let name = match str::from_utf8(&buf) {
            Ok(string) => string,
            Err(_) => "Random Crab",
        };
        self.name = Arc::new(String::from(truncate(name, read_size).trim()));
    }

    // Consumes itself
    pub fn handle(mut self) -> io::Result<()> {
        // Notify processor that client connected
        let _ = self
            .sender
            .send(MessageType::ClientConnected(self.stream.clone()));
        // Get name from user
        self.get_name();

        let mut stream = self.stream.as_ref();

        let _ = stream.write("Welcome to the crabver! Server for rust lovers!\n".as_bytes());
        let mut reader = BufReader::new(stream);

        loop {
            let mut input = String::new();
            let _ = reader.read_line(&mut input);
            let _ = self.sender.send(MessageType::Text(
                self.stream.clone(),
                self.name.clone(),
                input,
            ));
        }
    }
}

// Stolen from here: https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters
fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
