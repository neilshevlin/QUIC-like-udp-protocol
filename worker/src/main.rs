use std::thread;
use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::{env, str};

fn main(){
//    setup a worker server that takes a request from the server and respods to it
    let socket = UdpSocket::bind("127.0.0.1:4444").expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");

        match socket.recv_from(&mut buf){
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("handling conneciton from {}", src);
                    sock.send_to(&buf[..amt], &src).expect("Failed to send data");
                });
            }
            Err(e) => println!("Unable to recieve datagram: {}", e)
        }
    }
}
