use std::os::unix::net::UnixStream;
use std::io;
use std::io::prelude::*;

fn main() {
    match ping() {
        Ok(s) => println!("> {}", s),
        Err(e) => println!("something went wrong: {:?}", e),
    }
}

fn ping() -> io::Result<String> {
    let mut socket = UnixStream::connect("/var/run/docker.sock")?;
    socket.write_all(b"GET /_ping HTTP/1.1\r\nHost: docker\r\nConnection: close\r\n\r\n")?;
    
    let mut reader = io::BufReader::new(socket);
    let code = parse_http_status_code(&mut reader)?;
    println!("HTTP status code = {}", code);

    let mut response = String::new();
    reader.read_to_string(&mut response)?;
    return Ok(response);
}

fn parse_http_status_code(reader: &mut dyn io::BufRead) -> io::Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line)?;

    // HTTP/1.1 <code> <reason>
    let i = line.find(' ').unwrap();
    let code: String = line.chars().skip(i + 1).take(3).collect();
    return Ok(code)
}

