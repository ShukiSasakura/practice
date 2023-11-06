use std::net::{ToSocketAddrs, TcpStream};
use std::io::Read;

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
            for i in 0.. {
                let mut buf: [u8; 1024] = [1; 1024];
                let _ = stream.read_exact(&mut buf[..]).map_err(|e| {println!("{:?}, {:?}", e, buf.len()); e});
                if (i % 1000) == 0 {
                    println!("Receive message {}: {:?}", i, buf);
                    println!("{}",buf[0]);
                }
                if let Some((index, value)) = buf.iter().enumerate().find(|(_, &x)| x as usize != i % 256) {
                    println!("Error position:{}, {}, {}", i, index, value);
                    println!("Receive message {}: {:?}", i, buf[0]);
                    break;
                }
            }
        }
    }
}
