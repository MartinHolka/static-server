use crate::config::Config;
use crate::handler::handle_get;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn run(config: &Config) -> std::io::Result<()> {
    let listener = TcpListener::bind((&config.host[..], config.port))?;
    println!("Server running on {}:{}", config.host, config.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream, config) {
                    eprintln!("Connection handling failed: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, config: &Config) -> std::io::Result<()> {
    let mut buf = vec![0_u8; 512];

    stream.read(&mut buf).unwrap();
    let request_str = std::str::from_utf8(&buf).unwrap();
    let request_line = request_str.lines().next().unwrap_or("").trim();

    let mut parts = request_line.split(' ');

    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    let response: Vec<u8> = match method {
        "GET" => handle_get(path, &config.root_dir[..]),
        _ => "HTTP/1.1 400 Bad Request\r\n".as_bytes().to_vec(),
    };

    stream.write_all(&response)?;
    Ok(())
}
