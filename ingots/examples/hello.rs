#[macro_use]
extern crate ingots;


struct HelloWorld;

impl ingots::Ingot for HelloWorld {
    fn handle(&self, context: &mut ingots::http::Context) {
        context.response().set_header("Content-Type", "text/plain".into());
        write!(context.response(), "\r\nHello");
    }
}

ingot_init! {
    HelloWorld
}

fn main() {}
