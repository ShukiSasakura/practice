use std::io::Write;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(e) => return Err(e)
    };

    let buf = [1; 1024];
    let _ = stream.write(& buf);

    Ok(())
}
