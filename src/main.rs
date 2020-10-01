use std::os::unix::net::UnixStream;
use std::io;
use std::io::prelude::*;

fn main() {
    match ping() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("something went wrong: {:?}", e),
    }
}

fn ping() -> io::Result<String> {
    let mut socket = UnixStream::connect("/var/run/docker.sock")?;
    socket.write_all(b"GET /_ping HTTP/1.1\r\nHost: docker\r\nConnection: close\r\n\r\n")?;
    let mut response = String::new();
    socket.read_to_string(&mut response)?;
    return Ok(response);
}
