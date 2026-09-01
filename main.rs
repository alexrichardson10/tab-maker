mod binary_math;

use std::net::{SocketAddr, UdpSocket};
use binary_math::binary_math::*;

/* UNPROTECTED FIRST BYTE FORMAT (TYPICALLY PROTECTED)
MSB	1	Long header format
1	Fixed bit (always set)
00	Packet type: Initial
00	Reserved (always unset)
LSB	00	Packet Number field length (indicates the "Packet Number"
field below will have length of one byte) */

const ADDRESS: &str = "127.0.0.1:8080";
const STOP_CODE: [char; 4] = ['S', 'T', 'O', 'P'];
const BUFFER_SIZE: usize = 1200;
const SECRET_KEY: [u8; 32] = [160u8; 32];
//const P: u32 = 9;
const M: u32 = 255;

pub fn point_add(_a: &Vec<u8>, _b: &Vec<u8>) -> Vec<u8> {

    todo!()
}

pub fn point_double(_a: &Vec<u8>) -> Vec<u8> {

    todo!()
}

fn montgomery_ladder(mut k: [u8; 32]) -> [u8; 32] {
    let mut k_as_bits: [bool; 256] = [false; 256];

    for i in 0..32 {
        for j in 0..8 {
            k[i] = k[i].rotate_left(1);
            k_as_bits[i * 8 + j] = if k[i] % 2 == 1 { true } else { false };
        }
    }

    let mut r0: Vec<u8> = vec![0]; // infinity thing
    let mut r1: Vec<u8> = vec![1, 0, 0, 1]; // P as an array of bits

    for i in (0..M as usize).rev() {
        if k_as_bits[i] {
            r0 = point_add(&r0, &r0);
            r1 = point_double(&r1);

            r0 = binary_mod(&r0);
            r1 = binary_mod(&r1);
        }
        else {
            r1 = point_add(&r0, &r1);
            r0 = point_double(&r0);
        }

    }

    /* for i in (1..M).rev() {
        let j = i - 1;
        //println!("{j} ");
    } */

    [255u8; 32]
}

fn _parse_initial_packet(_length: usize, buf: [u8; BUFFER_SIZE]) {
    // determine initial keys


    let first_byte: u8 = buf[0];
    let is_long_header: bool = if first_byte % 2 == 1 { true } else { false };

    if !is_long_header {
        panic!("Short headers cannot be parsed.");
    }


}

fn make_socket() -> UdpSocket {
    let socket: UdpSocket = UdpSocket::bind(ADDRESS).unwrap();

    socket
}

fn test_socket(socket: &UdpSocket) {
    let mut stop: bool = false;

    while !stop {
        let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let (length, source): (usize, SocketAddr) = socket.recv_from(&mut buf).unwrap();

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

        /* for i in &buf[..length] {
            //print!("{:02x} ", i);
        } println!(); */

        println!("IP: {} Size: {} First Byte: {:08b}\n", source.ip(), length, buf[0]);

        let _garbage = socket.send_to(&buf, &source);
    }
}

fn main() {
    let a: [u8; 10] = [1, 1, 0, 1, 0, 1, 0, 0, 1, 0];
    let b: [u8; 4] = [1, 1, 1, 1];

    println!("add: {:?}", binary_add(&a.to_vec(), &b.to_vec()));
    println!("sub: {:?}", binary_sub(&a.to_vec(), &b.to_vec()));
    println!("mul: {:?}", binary_mul(&a.to_vec(), &b.to_vec()));
    println!("div: {:?}", binary_div(&a.to_vec(), &b.to_vec()));
    println!("mod: {:?}", binary_mod(&a.to_vec()));

    //montgomery_ladder(SECRET_KEY);
    //let socket: UdpSocket = make_socket();
    //test_socket(&socket);
}