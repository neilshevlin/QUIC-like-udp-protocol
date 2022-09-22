use std::thread;
use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::{env, str};

fn main(){
    // bind ot the local address on a given port and handle the error
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");

    loop {
        // There is no data to be read, so we will wait for it
        let mut buf = [0u8; 1500];// allocation of a static buffer
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