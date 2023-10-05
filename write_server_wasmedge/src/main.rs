use std::io;
use std::io::{Read, Write, ErrorKind};
use std::thread;
use std::time;
use wasmedge_wasi_socket::{ToSocketAddrs, TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5555", false).unwrap();
    let mut stream  = match listener.accept(true) {
        Ok((stream, _)) => stream ,
        Err(e) => return Err(e)
    };
    loop {
        send_msg(&mut stream);
        // thread::sleep(std::time::Duration::from_micros(10));
    }
}

// fn recv_msg(stream: &mut TcpStream) -> Result<(), std::io::Error> {
//     let mut buf: [u8; 2048] = [0; 2048];
//     stream.read_exact(&mut buf)?;
//     Ok(())
// }

fn send_msg(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let mut msg = "Send Message";
    let mut buf = [0u8; 1024];
    unsafe {
        let msg = msg.as_bytes();
        std::ptr::copy_nonoverlapping(msg.as_ptr(), buf.as_mut_ptr(), msg.len());
    }
    stream.write(& buf[..]);
    // stream.write_all(& buf[..]);
    Ok(())
}
