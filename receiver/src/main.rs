use std::net::{ToSocketAddrs, TcpListener, TcpStream};
use std::io::Read;
use std::str;

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
            println!("connect");
            for i in 0..256 {
                let mut buf: Vec<u8> = vec![0];
                // buf.resize(1024, 0);
                let _ = stream.read_exact(&mut buf);
                // buf.truncate("Send Message".as_bytes().len());
                // let msg = String::from_utf8(buf).unwrap();
                println!("Receive message {}: {:?}", i, buf);
            }
        }
    }
}
