extern crate toml;

use std::path::PathBuf;
use std::time::Duration;


#[derive(Clone)]
pub struct ServerConfig {
    pub keep_alive: Option<Duration>,
    pub read_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
    pub threads: usize,
    pub locations: Vec<Location>,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            keep_alive: None,
            read_timeout: None,
            write_timeout: None,
            threads: 1,
            locations: Vec::new(),
        }
    }
}

impl ServerConfig {
    pub fn add_location(&mut self, location: Location) {
        self.locations.push(location);
    }
}

#[derive(Clone)]
pub struct Location {
    pub prefix: String,
    pub ingot: PathBuf,
}
