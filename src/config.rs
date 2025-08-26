pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }
}

