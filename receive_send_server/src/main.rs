use std::io::{Read, Write};
use std::io::stdout;
// native, Wasmtime, Wasmer
use std::net::TcpListener;
// Wasmedge
// use wasmedge_wasi_socket::TcpListener;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut recv_timers:Vec<u64> = Vec::with_capacity(100000);
    let mut send_timers:Vec<u64> = Vec::with_capacity(100000);
    for _ in 1..=100000 {
        recv_timers.push(0);
        send_timers.push(0);
    }
    recv_timers.clear();
    send_timers.clear();
    let start_time = Instant::now();
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // let listener = TcpListener::bind("127.0.0.1:8080", false)?;
    // let _ = listener.set_nonblocking(true);
    let mut stream = match listener.accept() {
    // let mut stream = match listener.accept(true) {
        Ok((stream, _)) => stream,
        Err(e) => return Err(e)
    };
    let mut buf: [u8; 1024] = [0; 1024];
    // println!("start loop");
    loop {
        let _ = stream.read_exact(&mut buf);
        append_time(&mut recv_timers, start_time);
        if buf[0] == 2 {
            let _ = writeln!(stdout(), "receive times");
            recv_timers.iter()
                .enumerate()
                .for_each(|(_, record)| println!("{}", record));
            break;
        }
    }

    buf[0] = 3;

    for _ in 1..=100000 {
        let _ = stream.write_all(&buf);
        append_time(&mut send_timers, start_time);
    }

    let _ = writeln!(stdout(), "send times");
    send_timers.iter()
               .enumerate()
               .for_each(|(_, record)| println!("{}", record));

    Ok(())
}

fn append_time(timers:&mut Vec<u64>, start_time: Instant) {
    let elapsed = start_time.elapsed();
    let tsc = elapsed.as_nanos() as u64;
    timers.push(tsc);
}
