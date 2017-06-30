extern crate hyper;
extern crate ingots;
extern crate ingots_loader;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate toml;

mod config;
mod context;
mod engine;
mod server;


fn main() {
    let _ = simplelog::SimpleLogger::init(log::LogLevelFilter::Debug, simplelog::Config::default());

    let mut config = config::ServerConfig::default();
    config.add_location(config::Location {
        prefix: String::from("/"),
        ingot: "../target/debug/examples/libhello-2f78a9a51e5e3b87.so".into(),
    });

    let mut server = server::Server::new(config);
    server.listen();
}
