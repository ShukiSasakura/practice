use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

// wasmtime-cli WASI Preview 1
// use std::os::fd::*;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // wasmtime-cli WASI Preview 1
    // let listener = unsafe{TcpListener::from_raw_fd(3)};
    let _ = listener.set_nonblocking(false);
    loop {
        let mut stream = match listener.accept() {
            Ok((stream, _)) => stream,
            Err(e) => return Err(e)
        };
        thread::spawn(move || msg_read(stream));
        for i in 1..10 {
            println!("thread: main");
            println!("operation test {:?}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    // Ok(())
}

fn msg_read(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf);
    println!("read: {:?}", buf[0]);
    for i in 1..10 {
        println!("thread: spawned");
        println!("operation spawned: {:?}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
