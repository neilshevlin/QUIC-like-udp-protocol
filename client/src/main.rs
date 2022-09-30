use std::env;
use std::error::Error;
use std::io::{stdin, Read};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::{io};

struct Client {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}
impl Client {
    async fn run(self) -> Result<(), io::Error> {
        let Client {
            socket,
            buf: _,
            to_send,
        } = self;

        loop {
            if let Some((_size, _peer)) = to_send {
                let mut data = vec![0; 1350];
                let len = socket.recv(&mut data).await?;
                println!(
                    "Received {} bytes:\n{}",
                    len,
                    String::from_utf8_lossy(&data[..len])
                );
            }

            let mut input  = String::new();
            io::stdin().read_line(&mut input)?;
            socket.send(input.as_bytes()).await?;
            let mut data = vec![0; 1350];
            let len = socket.recv(&mut data).await?;
            println!(
                "Received {} bytes:{}",
                len,
                String::from_utf8_lossy(&data[..len])
            );
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _remote_addr: SocketAddr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".into())
        .parse()?;

    // We use port 0 to let the operating system allocate an available port for us.
    let _local_addr: SocketAddr = env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:8000".into())
        .parse()?;

    let socket = UdpSocket::bind(_local_addr).await?;
    println!("Listening on: {}", socket.local_addr()?);
    const MAX_DATAGRAM_SIZE: usize = 65_507;
    socket.connect("127.0.0.1:8080").await?;

    let client = Client {
        socket,
        buf: vec![0; 1350],
        to_send: None,
    };

    client.run().await?;
    Ok(())
}





