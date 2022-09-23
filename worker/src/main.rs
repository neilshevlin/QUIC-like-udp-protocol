use std::thread;
use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::{env, str};

fn main(){
//    setup a worker server that takes a request from the server and respods to it
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");

        match socket.recv_from(&mut buf){
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("Recieved: {}", str::from_utf8(&buf).expect("Could not write buffer as string"));
                    sock.send_to(b"THIS WORKER HAS ACKNOWLEDGED YOUR REQUEST", src).expect("Failed to send data");
                });
            }
            Err(e) => println!("Unable to recieve datagram: {}", e)
        }
    }
}

async fn handle_work_request(socket: UdpSocket, src: SocketAddr, buf: &[u8], amt: usize){

    socket.send_to(b"THIS WORKER HAS ACKNOWLEDGED YOUR REQUEST", src).expect("Failed to send data");

    // wait five seconds to simulate a delay
    thread::sleep(std::time::Duration::from_secs(5));

    socket.send_to(b"THIS WORKER HAS COMPLETED YOUR REQUEST", src).expect("Failed to send data");

}

