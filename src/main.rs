use std::io::{BufRead, BufReader, Read, Write};
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
                thread::spawn(move || handle_client(stream));
                ()
            }
            Err(e) => eprintln!("Couldn't accept because of {}", e),
        }
    }
}

const MESSAGE: &'static str = "Hello there stranger\n";

enum MessageError {
    WrongSyntax,
}

fn handle_client(stream: Arc<TcpStream>) -> ! {
    let mut stream = stream.as_ref();
    let mut reader = BufReader::new(stream);
    let _ = stream.write(MESSAGE.as_bytes());

    loop {
        let mut input = String::new();
        let _ = reader.read_line(&mut input);
        let response = match process_message(&input) {
            Ok(message) => message,
            Err(e) => match e {
                MessageError::WrongSyntax => "Malformed Message\n".to_owned(),
            },
        };
        let _ = stream.write(response.as_bytes());
    }
}

fn process_message(message: &str) -> Result<String, MessageError> {
    let mut splitted = message.split(' ');
    let mut command = splitted.nth(0).unwrap().trim().to_owned();
    command.push_str(" to you too!\n");
    Ok(command)
}
