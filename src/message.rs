use std::net::TcpStream;
use std::sync::Arc;

pub enum MessageType {
    ClientConnected(Arc<TcpStream>),
    Text(Arc<TcpStream>, Arc<String>, String),
    ClientDisconnected(Arc<TcpStream>),
}
