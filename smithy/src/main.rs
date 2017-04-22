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
    let _ = simplelog::SimpleLogger::init(log::LogLevelFilter::Info, simplelog::Config::default());

    let mut config = config::ServerConfig::default();
    config.add_location(config::Location {
        prefix: String::from("/"),
        ingot: "/Users/scoakley/dev/sagebind/ingots/ingots-loader/target/debug/examples/libhello-2cb25cd11455e06f.dylib".into(),
    });

    let mut server = server::Server::new(config);
    server.listen();
}
