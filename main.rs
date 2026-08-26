use std::net::{SocketAddr, UdpSocket};

const ADDRESS: &str = "127.0.0.1:8080";
const STOP_CODE: [char; 4] = ['S', 'T', 'O', 'P'];

fn make_socket() -> UdpSocket {
    let socket: UdpSocket = UdpSocket::bind(ADDRESS).unwrap();

    socket
}

fn test_socket(socket: &UdpSocket) {
    let mut stop: bool = false;

    while !stop {
        let mut buf: [u8; 1024] = [0; 1024];
        let (_length, source): (usize, SocketAddr) = socket.recv_from(&mut buf).unwrap();

        let mut matches = 0;
        for item in STOP_CODE {
            if item == char::from(buf[matches]) {
                matches += 1;
            } else {
                break;
            }
        }

        if matches == STOP_CODE.len() {
            stop = true;
        }

        println!("{}\n", source.ip());

        let _garbage = socket.send_to(&buf, &source);
    }
}

fn main() {
    let socket: UdpSocket = make_socket();
    test_socket(&socket);
}