extern crate ingots_loader;
extern crate ingots_fastcgi;

use ingots_loader::*;


fn main() {
    let ingot = DynamicIngot::open("target/debug/examples/libhello-2cb25cd11455e06f.dylib")
        .unwrap();

    let server = ingots_fastcgi::Server::new(ingot);
    server.listen_tcp("localhost:9000");
}
