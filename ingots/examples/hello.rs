extern crate ingots;


struct HelloWorld;

impl ingots::Ingot for HelloWorld {
    fn handle(&self, context: &mut ingots::http::Context) {
        context.response().write_header("Content-Type", "text/plain");
        write!(context.response(), "\r\nHello");
    }
}

#[no_mangle]
pub extern fn ingot_entrypoint() -> ingots::IngotBox {
    HelloWorld.into()
}

fn main() {}
