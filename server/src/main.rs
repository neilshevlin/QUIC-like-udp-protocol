use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}
impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            if let Some((size, peer)) = to_send {
                let res = handle_client(&socket, &buf, size, peer).await;
                println!("Sent response: {:?}", res);
            }
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
// async handle_client function that returns a future
async fn handle_client(socket: &UdpSocket, buf: &Vec<u8>, size: usize, peer: SocketAddr) -> Result<(), Box<dyn Error>> {

    if valid_request(&buf[..size]) {
        let _amt = socket.send_to(b"Recieved request", &peer).await?;
        println!("VALID REQUEST - Sent acknowledgemnt to {}", peer);
        

    } else {
        let _amt = socket.send_to(b"Invalid request recieved", &peer).await?;
        println!("INVALID REQUEST - Sent acknowledgemnt to {}", peer);
    }

    Ok(())
}

fn valid_request(buf: &[u8]) -> bool {
    if std::str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("GET"){
        return true;
    }
    else if std::str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("POST"){
        return true;
    }
    else if std::str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("PUT"){
        return true;
    }
    else if std::str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("DELETE"){
        return true;
    }
    else if std::str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("PATCH"){
        return true;
    }
    else {
        return false;
    }
}


