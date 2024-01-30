use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(e) => return Err(e)
    };

    let mut buf = [1; 1024];

    for i in 1..=100000 {
        if i != 100000 {
            let _ = stream.write(& buf);
        } else {
            buf[0] = 2;
            let _ = stream.write(& buf);
        }
    }

    for i in 1..=100000 {
        let _ = stream.read(&mut buf);
    }

    Ok(())
}
