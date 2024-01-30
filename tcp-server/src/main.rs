use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

// WASI Preview 1 で Wasmtime ランタイムを使用する場合に使用する
// use std::os::fd::*;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // WASI Preview 1 で Wasmtime ランタイムを使用する場合に使用する
    // let listener = unsafe{TcpListener::from_raw_fd(3)};
    let _ = listener.set_nonblocking(false);
    loop {
        let mut stream = match listener.accept() {
            Ok((stream, _)) => stream,
            Err(e) => return Err(e)
        };
        thread::spawn(move || treat_msg(stream));
        for i in 1..10 {
            // println!("thread: main");
            // println!("operation test {:?}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    // Ok(())
}

fn treat_msg(mut stream: TcpStream) {
    let mut buf: [u8; 1024] = [0; 1024];
    buf = msg_read(&mut stream);
    msg_send(&mut stream, buf);
}

fn msg_read(stream: &mut TcpStream) -> [u8; 1024] {
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf);
    println!("read: {:?}", buf[0]);
    for i in 1..10 {
        // println!("thread: spawned");
        // println!("operation spawned: {:?}", i);
        thread::sleep(Duration::from_millis(1));
    }
    return buf
}

fn msg_send(stream: &mut TcpStream, mut buf: [u8; 1024]) {
    let buf: [u8; 1024] = [0; 1024];
    let _ = stream.write_all(& buf);
    println!("send: {:?}", buf[0]);
}
