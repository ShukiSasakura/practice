use std::io;
use std::io::Write;
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
        let _ = send_msg(&mut stream, &msg);
        // thread::sleep(std::time::Duration::from_micros(10));
        i += 1;
    }
}

fn send_msg(stream: &mut TcpStream, msg: &[u8]) -> Result<(), std::io::Error> {
    // let msg = msg.as_bytes();
    // let _ = stream.write(& msg);
    let _ = stream.write_all(msg);
    Ok(())
}
