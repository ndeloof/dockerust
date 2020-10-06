use std::os::unix::net::UnixStream;
use std::io;
use std::io::prelude::*;
use http::{HeaderMap, HeaderValue};
use http::header::HeaderName;
use std::str::FromStr;

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

    parse_http_headers(&mut reader);
    let mut response = String::new();
    return Ok(response);
}

fn parse_http_status_code(reader: &mut impl io::BufRead) -> io::Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line)?;

    // HTTP/1.1 <code> <reason>
    let i = line.find(' ').unwrap();
    let code: String = line.chars().skip(i + 1).take(3).collect();
    return Ok(code)
}

fn parse_http_headers(reader: &mut impl io::BufRead) -> io::Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        line = line.trim().to_string();
        if line.len() == 0 {
            return Ok(headers)
        }
        let parts: Vec<&str> = line.split(':').collect();
        println!(" {} = {}", parts[0], parts[1]);
        let name = HeaderName::from_str(parts[0]).unwrap();
        let value = HeaderValue::from_str(parts[1]).unwrap();
        headers.append(name, value);
    }
}
