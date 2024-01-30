use std::io::{Read, Write};
use std::io::stdout;
// native, Wasmtime, Wasmer
// use std::net::{TcpListener, TcpStream};
// Wasmedge
use wasmedge_wasi_socket::TcpListener;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start_time = Instant::now();
    let mut timers:Vec<u64> = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:8080", false)?;
    // let _ = listener.set_nonblocking(false);
    let mut stream = match listener.accept(true) {
        Ok((stream, _)) => stream,
        Err(e) => return Err(e)
    };
    // println!("start loop");
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let _ = stream.read_exact(&mut buf);
        match buf[0] {
            1 => {
                // println!("pattern 1");
                append_time(&mut timers, start_time);
            },
            2 => {
                // println!("pattern 2");
                timers.iter()
                      .enumerate()
                      .for_each(|(_, record)| writeln!(stdout(), "{}", record).unwrap())
            },
            _ =>{},
        }
    }
}

fn append_time(timers:&mut Vec<u64>, start_time: Instant) {
    let elapsed = start_time.elapsed();
    let tsc = elapsed.as_nanos() as u64;
    timers.push(tsc);
}
