use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use utils::simple_input::get_user_input;

#[tokio::main]
async fn main() {
    let mut stream = tokio::net::TcpStream::connect("localhost:8080")
        .await
        .unwrap();
    let (read, write) = stream.into_split();
    tokio::spawn(async move { handle_read(read).await });
    handle_write(write).await
}

async fn handle_read(mut read: OwnedReadHalf) {
    loop {
        let mut buf1 = [0u8; 8];
        match read.read_exact(&mut buf1).await {
            Err(_) => continue,
            Ok(_) => {}
        }
        let size = u64::from_be_bytes(buf1) as usize;
        let mut msgbuf = vec![0u8; size];
        read.read_exact(&mut msgbuf).await.unwrap();

        println!("{}", String::from_utf8(msgbuf).unwrap())
    }
}

async fn handle_write(mut write: OwnedWriteHalf) {
    loop {
        let msg = get_user_input();

        write.write_all(&msg.len().to_be_bytes()).await.unwrap();
        write.write_all(&msg.as_bytes()).await.unwrap();
    }
}
