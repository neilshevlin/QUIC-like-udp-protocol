use std::net::UdpSocket;
use std::{str, io};
//CLIENT
fn main(){
    let socket = UdpSocket::bind("0.0.0.0:8000").expect("Could not bind socket");

    socket.connect("0.0.0.0:8080").expect("Could not connect to server");

    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];
        match io::stdin().read_line(&mut input){
            Ok(_) => {
                socket.send(input.as_bytes()).expect("Failed to send data");
                socket.recv_from(&mut buffer).expect("Failed to recieve data");
                println!("Response from server: {}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
            }
            Err(e) => println!("Unable to read line: {}", e)
        }
    }
}