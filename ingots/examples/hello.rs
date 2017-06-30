#[macro_use]
extern crate ingots;


struct HelloWorld;

impl ingots::Ingot for HelloWorld {
    fn handle(&self, context: &mut ingots::http::Context) {
        context.response().write_header("Content-Type", "text/plain");
        write!(context.response(), "\r\nHello");
    }
}

ingot_init! {
    HelloWorld
}

fn main() {}
