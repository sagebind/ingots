extern crate ingots;


struct HelloWorld;

impl ingots::Ingot for HelloWorld {
    fn handle(&self, context: &mut ingots::Context) {
        context.response().write_header("Content-Type", "text/plain");
        write!(context.response(), "\r\nHello");
    }
}

#[no_mangle]
pub extern fn ingot_entrypoint() -> Box<ingots::Ingot> {
    Box::new(HelloWorld)
}

fn main() {}
