use distributed_rust::build_connection;
use distributed_rust::send_message;

use std::net::{
    Ipv4Addr,
    SocketAddrV4,
    UdpSocket,
};

use std::io;

fn main(){

    let ip = std::net::Ipv4Addr::new(127, 0, 0, 1);
    let server_port = 8080;

    let server_addr = build_connection(ip, server_port);

    let client_addr = build_connection(ip, 0);

    let socket = UdpSocket::bind(client_addr)
    .expect("Failed to bind client socket ");

    println!("Client started on {}:{}", ip, socket.local_addr().unwrap().port());

    loop {
        println!("Enter a message to send to the server (type 'exit' to quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let message = input.trim();
        
        if message.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        send_message(&socket, server_addr, message);
    }
    

}