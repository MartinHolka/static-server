use crate::config::{self, Config};
use std::net::{TcpListener, TcpStream};

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

fn handle_connection(stream: TcpStream, config: &Config) {}
