use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use utils::message::Message;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = net::TcpListener::bind("0.0.0.0:8080").await?;
    let (sender, receiver) = broadcast::channel(100);
    loop {
        let (socket, _) = listener.accept().await?;
        let send = sender.clone();

        let recv = sender.clone().subscribe();

        println!("Got connection from {:?}", socket.peer_addr().unwrap());

        let (read, write) = socket.into_split();

        tokio::spawn(async move { handle_read(read, send).await });
        tokio::spawn(async move { handle_write(write, recv).await });
    }
}

async fn handle_connection(
    mut socket: net::TcpStream,
    recv: Receiver<String>,
    send: Sender<String>,
) {
}

async fn handle_read(mut read: OwnedReadHalf, mut sender: Sender<Message>) {
    loop {
        let mut buf1 = [0u8; 8];
        match read.read_exact(&mut buf1).await {
            Err(_) => continue,
            Ok(_) => {}
        }
        let size = u64::from_be_bytes(buf1) as usize;
        let mut msgbuf = vec![0u8; size];
        read.read_exact(&mut msgbuf).await.unwrap();
        let strversion = String::from_utf8(msgbuf).unwrap();
        let msg = Message {
            ip: read.peer_addr().unwrap(),
            content: strversion,
        };
        sender.send(msg).unwrap();
    }
}

async fn handle_write(mut write: OwnedWriteHalf, mut recv: Receiver<Message>) {
    loop {
        let msg = recv.recv().await.unwrap();
        let bytes = msg.content.as_bytes();
        let ip = msg.ip;
        if ip != write.peer_addr().unwrap() {
            write
                .write_all(&mut bytes.len().to_be_bytes())
                .await
                .unwrap();
            write.write_all(&bytes).await.unwrap();
        }
    }
}
