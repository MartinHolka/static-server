pub struct Config {
    pub host: String,
    pub port: u16,
    pub root_dir: String,
}

impl Config {
    pub fn new(host: &str, port: u16, root_dir: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            root_dir: root_dir.to_string(),
        }
    }
}
