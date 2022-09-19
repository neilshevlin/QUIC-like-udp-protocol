use std::thread;
use std::net::{UdpSocket, SocketAddr};

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
                    handle_client(sock, src, &buf, amt);
                });
            }
            Err(e) => println!("Unable to recieve datagram: {}", e)
        }
    }
}
fn handle_client(mut socket: UdpSocket, src: SocketAddr, buf: &[u8], amt: usize){
    // read in the file sent by the client as a String
    let data = String::from_utf8_lossy(&buf[..amt]);

    // list the files in the current directory
    let files = std::fs::read_dir(".").expect("Could not read current directory");
    // print all the files
    for file in files {
        println!("{:?}", file);
    }
    // return back teh contents of the file test.txt in the current directory
    let contents = std::fs::read_to_string("test.txt").expect("Could not read file");
    socket.send_to(contents.as_bytes(), src).expect("Could not send data to client");
}
