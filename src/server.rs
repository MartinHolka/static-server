use crate::config::{self, Config};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

pub fn run(config: Config) -> std::io::Result<()> {
    let listener = TcpListener::bind((&config.host[..], config.port))?;
    println!("Server running on {}:{}", config.host, config.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, &config),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, config: &Config) {
    let mut buf = vec![0_u8; 512];

    stream.read(&mut buf).unwrap();
    let request_str = std::str::from_utf8(&buf).unwrap();
    println!("{}", request_str);
}
