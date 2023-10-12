use std::net::{ToSocketAddrs, TcpListener, TcpStream};
use std::io::Read;
// use std::str;

fn main() {
    let host_and_port = "localhost:5555";
    let mut addrs = host_and_port.to_socket_addrs().unwrap();

    let Some(addr) = addrs.find(|x| (*x).is_ipv4()) else {
        eprintln!("Invalid Host:Port Number");
        return;
    };

    match TcpStream::connect(addr) {
        Err(_) => {
            println!("Failed connect");
        }
        Ok(mut stream) => {
            let mut i: u32 = 0;
            // println!("connect");
            loop {
                // let mut buf: Vec<u8> = vec![0];
                let mut buf: [u8; 1] = [0];
                let _ = stream.read_exact(&mut buf);
                // if (i % 1000000) == 0 {
                    println!("Receive message {}: {:?}", i, buf[0]);
                    // println!("{}",buf[0]);
                // }
                if (i % 256) != buf[0].into() {
                    // println!("Receive message {}: {:?}", i, buf[0]);
                    // println!("Error-----------------------------------");
                    break;
                }
                i += 1;
            }
        }
    }
}
