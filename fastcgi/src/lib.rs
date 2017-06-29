extern crate fastcgi;
extern crate ingots;

mod context;

use ingots::*;
use std::net::{TcpListener, ToSocketAddrs};


/// Wraps a Rust ingot in a FastCGI server.
pub struct Server<I: Ingot> {
    ingot: I,
}

impl<I: Ingot> Server<I> {
    pub fn new(ingot: I) -> Server<I> {
        Server {
            ingot: ingot,
        }
    }

    /// Listen for requests over a UNIX socket.
    pub fn listen_unix(&self) {
        fastcgi::run(|request| {
            self.handle_request(request);
        });
    }

    /// Listen for requests over a TCP socket.
    pub fn listen_tcp<A: ToSocketAddrs>(&self, addr: A) {
        let listener = TcpListener::bind(addr).unwrap();

        fastcgi::run_tcp(|request| {
            self.handle_request(request);
        }, &listener);
    }

    fn handle_request(&self, request: fastcgi::Request) {
        let mut context = context::Context::from(request);

        self.ingot.handle(&mut context);
    }
}
