extern crate tiny_http;
extern crate ingots;
extern crate ingots_loader;
#[macro_use]
extern crate log;
extern crate simplelog;

mod adapter;

use std::env;
use tiny_http::{Server, Request, Response};


fn main() {
    let _ = simplelog::SimpleLogger::init(log::LogLevelFilter::Debug, simplelog::Config::default());
    let mut ingot;

    if let Some(path) = env::args().nth(1) {
        ingot = ingots_loader::DynamicIngot::open(path).expect("failed to load ingot");
    } else {
        warn!("no ingot file given");
        return;
    }

    let server = Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        let mut context = adapter::Context {
            server_addr: server.server_addr(),
            request: request,
            response: Response::from_data(Vec::new()),
        };

        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let response = Response::from_string("hello world");
        request.respond(response);
    }
}
