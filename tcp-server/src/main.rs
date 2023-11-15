use std::io::Read;
use std::net::TcpListener;

use std::os::fd::*;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // let listener = unsafe{TcpListener::from_raw_fd(3)};
    let _ = listener.set_nonblocking(false);
    let mut stream = match listener.accept() {
        Ok((stream, _)) => stream,
        Err(e) => return Err(e)
    };

    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf);
    println!("read: {:?}", buf);
    Ok(())

}
