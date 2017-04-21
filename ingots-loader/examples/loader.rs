extern crate ingots_loader;
extern crate ingots_fastcgi;

use ingots_loader::*;


fn main() {
    let mut ingot = DynamicIngot::new("/Users/scoakley/dev/sagebind/ingots/ingots-loader/target/debug/examples/libhello-2cb25cd11455e06f.dylib");
    ingot.reload();

    let server = ingots_fastcgi::Server::new(ingot);
    server.listen_tcp("localhost:9000");
}
