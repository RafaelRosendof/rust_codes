use std::net::{
    Ipv4Addr,
    UdpSocket,
};

use distributed_rust::build_connection;
use distributed_rust::receive_message;


fn main(){

    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8080;
    let socket_addr = build_connection(ip, port);

    let socket = UdpSocket::bind(socket_addr)
    .expect("Failed to bind socket");
    println!("[UDP] Server started on {}:{}", ip, port);

    loop {

        let (message, client_addr) = receive_message(&socket);
        println!("Received message from {}: {}", client_addr, message);
    }
}   

