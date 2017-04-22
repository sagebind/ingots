extern crate hyper;
extern crate ingots;
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
        ingot: "../ingots/target/debug/examples/libhello-36268a9f5f7a7dea.so".into(),
    });

    let mut server = server::Server::new(config);
    server.listen();
}
