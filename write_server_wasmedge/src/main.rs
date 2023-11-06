use std::io;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::time;
use wasmedge_wasi_socket::{ToSocketAddrs, TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5555", false).unwrap();
    let mut stream  = match listener.accept(true) {
        Ok((stream, _)) => stream ,
        Err(e) => return Err(e)
    };

    let mut counter = 0;

    for i in (0..=255).cycle().take(256 * 400) {
        let msg: [u8; 1024] = [i; 1024];
        let mut recv_buf: [u8; 1024] = [0; 1024];
        match stream.read_exact(&mut recv_buf) {
            Ok(_) => {},
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                 send_msg(&mut stream, &msg, &mut counter)?;
                }
            }
        }
    }
    let probability: f32 = (counter as f32/(256.0 * 400.0))*100.0;
    println!("WouldBlock num: {}, probability: {}", counter, probability);
    Ok(())
}

fn send_msg(stream: &mut TcpStream, msg: &[u8], counter: &mut u32) -> Result<(), std::io::Error> {
    // let msg = msg.as_bytes();
    // stream.write_all(msg);
    write_all(stream, msg, counter)?;
    Ok(())
}

fn write_all(stream: &mut TcpStream, mut buf: &[u8], counter: &mut u32) -> Result<(), std::io::Error> {
    let mut i = 0;
    let mut fib1 = 0;
    let mut fib2 = 1;
    while !buf.is_empty() {
        match stream.write(buf) {
            Ok(0) => {
                println!("send 0 size message");
                break;
            }
            Ok(1024) => {
                buf = &buf[1024..];
            }
            Ok(n) => {
                buf = &buf[n..];
            }
            Err(e) => {
                //"Resource temporarily unavailable (os error6)のときに再送"
                if e.kind() == io::ErrorKind::WouldBlock {
                    i += 1;
                    if i == 1 {
                        *counter += 1;
                    }
                    if i > 1 {
                        thread::sleep(std::time::Duration::from_micros(fib1+fib2));
                        let mut temp = fib1+fib2;
                        println!("{}", i);
                        fib1 = fib2;
                        fib2 = temp;
                    }
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}
