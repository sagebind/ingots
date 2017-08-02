#[macro_use]
extern crate ingots;

use ingots::http::header::HeaderValue;


struct HelloWorld;

impl ingots::Ingot for HelloWorld {
    fn handle(&self, context: &mut ingots::Context) {
        context.response().headers_mut().insert("Content-Type", HeaderValue::from_static("text/plain"));

        write!(context.response().body_mut(), "\r\nHello").unwrap();
    }
}

ingot_init! {
    HelloWorld
}

fn main() {}
