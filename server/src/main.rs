use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;
use std::time::Duration;
use tokio::io::AsyncReadExt;
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
                if res.is_err() {
                    println!("Error handling client: {:?}", res);
                }
                else {
                    let payload_file = res.unwrap();
                    let payload = payload_file.as_slice();
                    socket.connect(peer).await?;
                    let ans = socket.send(payload).await?;
                    println!("Sent response to : {}", peer);
                }

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


// returns data which is a vex[0;1350]
async fn worker_dispatch(socket: &UdpSocket, buf: &[u8], size: usize, peer: SocketAddr) -> Result<Vec<u8>, Box<dyn Error>> {

    let worker_addr: SocketAddr = env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:8888".into())
        .parse()?;

    
    // connect to a worker socket
    socket.connect(worker_addr).await?;
    
    let test_str = "test_step";
    socket.send(test_str.as_bytes()).await?;
    // wait for the worker to send back the file
    let mut data = vec![0; 1350];
    let len = socket.recv(&mut data).await?;
    println!(
        "Received {} bytes:{}",
        len,
        String::from_utf8_lossy(&data[..len])
    );
    // send the file back to the client
    println!("Sending response to: {}", peer);

    Ok(data)

}
// async function that returns a file in binary format



// async handle_client function that returns a future
async fn handle_client(socket: &UdpSocket, buf: &Vec<u8>, size: usize, peer: SocketAddr) -> Result<Vec<u8>, Box<dyn Error>> {

    if valid_request(&buf[..size]) {
        // dispatch to get file
        let file = worker_dispatch(socket, &buf, size, peer).await?;
        Ok(file)

    } else {
        let _amt = socket.send_to(b"Invalid request recieved", &peer).await?;
        println!("INVALID REQUEST - Sent acknowledgemnt to {}", peer);
    
        Ok(vec![0; 0])
    }


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


