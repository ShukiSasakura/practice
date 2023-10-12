use std::io;
use std::io::Write;
use std::println;
use std::thread;
use std::time;
use wasmedge_wasi_socket::{ToSocketAddrs, TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let mut i = 0;
    let listener = TcpListener::bind("127.0.0.1:5555", false).unwrap();
    let mut stream  = match listener.accept(true) {
        Ok((stream, _)) => stream ,
        Err(e) => return Err(e)
    };
    loop {
        let msg: [u8; 1] = [i];
        send_msg(&mut stream, &msg)?;
        // thread::sleep(std::time::Duration::from_micros(10));
        if i == 255 {
            i = 0;
        } else {
            i += 1;
        }
    }
}

fn send_msg(stream: &mut TcpStream, msg: &[u8]) -> Result<(), std::io::Error> {
    // let msg = msg.as_bytes();
    // let _ = stream.write(msg);
    // stream.write_all(msg)?;
    write_all(stream, msg)?;
    Ok(())
}

fn write_all(stream: &mut TcpStream, mut buf: &[u8]) -> Result<(), std::io::Error> {
        while !buf.is_empty() {
            match stream.write(buf) {
                Ok(0) => {
                    println!("send 0 size message");
                    break;
                }
                Ok(n) => buf = &buf[n..],
                Err(e) => {
                    //"Resource temporarily unavailable (os error6)のときにsleep"
                    if e.kind() != io::ErrorKind::WouldBlock {
                        return Err(e);
                    } else {
                        println!("WouldBlock");
                        // println!("sleep");
                        // thread::sleep(std::time::Duration::from_micros(10));
                    }
                }

            }
        }
        Ok(())
    }
