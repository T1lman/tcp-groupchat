use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub ip: SocketAddr,
}
