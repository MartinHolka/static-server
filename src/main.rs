mod config;
mod handler;
mod server;

use config::Config;
use server::run;

fn main() {
    let config = Config::new("127.0.0.1", 8080, "public");

    if let Err(e) = run(&config) {
        eprintln!("Server error: {}", e);
    }
}
