use std::thread;
use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::{env, str};

//SERVER
fn main(){
    // bind ot the local address on a given port and handle the error
    let server_socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");

    loop {
        // There is no data to be read, so we will wait for it
        let mut buf = [0u8; 1500];// allocation of a static buffer
        let server_sock = server_socket.try_clone().expect("Failed to clone socket");

        match server_sock.recv_from(&mut buf){
            Ok((amt, src)) => {
                thread::spawn(move || {
                    handle_client(server_sock, src, &buf, amt);
                });
            }
            Err(e) => println!("Unable to recieve datagram: {}", e)
        }
    }
}

fn simulate_delay_dispatch(server_sock: UdpSocket, src: SocketAddr, buf: &[u8], amt: usize) {
    thread::sleep(std::time::Duration::from_secs(5));
    server_sock.send_to(b"Server has sent you a response", src).expect("Failed to send data");
}

fn dispatch_to_worker(socket: UdpSocket, src: SocketAddr, buf: &[u8], amt: usize){
    socket.connect("0.0.0.0:8080").expect("Could not connect to worker");

    let mut buffer = [0u8; 1500];
    socket.send(&buf[..amt]).expect("Failed to send data");
    
    let mut buffer = [0u8; 1500];
    socket.recv_from(&mut buffer).expect("Failed to recieve data");
    println!("Acknowledgement from worker: {}", str::from_utf8(&buffer).expect("Could not write buffer as string"));

}

fn handle_client(mut server_sock: UdpSocket, src: SocketAddr, buf: &[u8], amt: usize){

    if valid_request(&buf){
        println!("Recieved: {}", str::from_utf8(&buf).expect("Could not write buffer as string"));
        server_sock.send_to(b"Recieved request", src).expect("Failed to send data");
        dispatch_to_worker(server_sock, src, &buf, amt);

    } else {
        let message = "Invalid request";
        server_sock.send_to(message.as_bytes(), src).expect("Could not send message");
    }
}
fn valid_request(buf: &[u8]) -> bool{
    if str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("GET"){
        return true;
    }
    else if str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("POST"){
        return true;
    }
    else if str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("PUT"){
        return true;
    }
    else if str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("DELETE"){
        return true;
    }
    else if str::from_utf8(&buf).expect("Could not write buffer as string").starts_with("PATCH"){
        return true;
    }
    else {
        return false;
    }
}