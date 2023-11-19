use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(address).unwrap();
    println!("Listenting on: {}", address);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                thread::spawn(move || handleClient(stream));
                ()
            }
            Err(e) => eprintln!("Couldn't accept because of {}", e),
        }
    }
}

const message: &'static str = "Hello there stranger\n";

fn handleClient(stream: Arc<TcpStream>) {
    let _ = stream.as_ref().write(message.as_bytes());
}
