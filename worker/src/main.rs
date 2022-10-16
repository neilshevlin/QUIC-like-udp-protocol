use std::env;
use std::error::Error;
use std::io::{stdin, Read};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::{io};
use std::time::Duration;
use tokio::io::AsyncReadExt;
struct Worker {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}
impl Worker {
    async fn run(self) -> Result<(), io::Error> {
        let Worker {
            socket,
            mut buf, 
            mut to_send,
        } = self;

        loop { 
            if let Some((size, peer)) = to_send {
                tokio::time::sleep(Duration::from_secs(2)).await;
                let mut file = tokio::fs::File::open("test.txt").await?;
                let mut buf = vec![0; 1024];
                file.read(&mut buf).await?;
                socket.send_to(&buf, peer).await?;
                println!("Sent response to : {}", peer);
            }
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());
    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let worker = Worker {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    worker.run().await?;

    Ok(())

}

// fn main(){
// //    setup a worker server that takes a request from the server and respods to it
//     let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");

//     loop {
//         let mut buf = [0u8; 1500];
//         let sock = socket.try_clone().expect("Failed to clone socket");

//         match socket.recv_from(&mut buf){
//             Ok((amt, src)) => {
//                 thread::spawn(move || {
//                     println!("Recieved: {}", str::from_utf8(&buf).expect("Could not write buffer as string"));
//                     sock.send_to(b"THIS WORKER HAS ACKNOWLEDGED YOUR REQUEST", src).expect("Failed to send data");
//                 });
//             }
//             Err(e) => println!("Unable to recieve datagram: {}", e)
//         }
//     }
// }

// async fn handle_work_request(socket: UdpSocket, src: SocketAddr, buf: &[u8], amt: usize){

//     socket.send_to(b"THIS WORKER HAS ACKNOWLEDGED YOUR REQUEST", src).expect("Failed to send data");

//     // wait five seconds to simulate a delay
//     thread::sleep(std::time::Duration::from_secs(5));

//     socket.send_to(b"THIS WORKER HAS COMPLETED YOUR REQUEST", src).expect("Failed to send data");

// }

