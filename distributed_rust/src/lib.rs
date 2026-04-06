use std::net::{
    Ipv4Addr,
    SocketAddrV4,
    UdpSocket,
};

pub fn build_connection(ip: Ipv4Addr, port: u16) -> SocketAddrV4{
    SocketAddrV4::new(ip, port)
}

pub fn send_message(socket: &UdpSocket, out_socket: SocketAddrV4, message: &str){
    
    let bytes_send = socket.send_to(message.as_bytes(), out_socket)
    .expect("Failed to send message");

    if bytes_send == message.len() {
        println!("Message sent successfully");
    } else {
        println!("Failed to send the entire message");
    }

}

pub fn receive_message(socket: &UdpSocket) -> (String, SocketAddrV4){

    let mut buffer = [0; 1024];

    let (size, src_adr) = socket
    .recv_from(&mut buffer)
    .expect("Failed to receive message");

    let message = String::from_utf8_lossy(&buffer[..size]).to_string();

    let send_addr = match src_adr{
        std::net::SocketAddr::V4(v4) => v4,
        _ => panic!("Expected an IPv4 address"),
    };

    (message, send_addr)
}


pub fn heartbeat(socket: &UdpSocket, out_socket: SocketAddrV4){

    let message = "alive";

    //TODO
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
