extern crate ingots;
extern crate ingots_fastcgi;


struct HelloWorld;

impl ingots::Ingot for HelloWorld {
    fn handle(&self, context: &mut ingots::Context) {
        context.response().write_header("Content-Type", "text/plain");
        write!(context.response(), "\r\n");

        let uri = context.request().uri();
        writeln!(context.response(), "uri: {}", uri);

        let query = context.request().query_string();
        writeln!(context.response(), "query: {:?}", query);

        let server_name = context.server_name();
        writeln!(context.response(), "server name: {:?}", server_name);

        let server_addr = context.server_addr();
        writeln!(context.response(), "server addr: {:?}", server_addr);

        let remote_addr = context.remote_addr();
        writeln!(context.response(), "remote addr: {:?}", remote_addr);
    }
}

fn main() {
    let server = ingots_fastcgi::Server::new(HelloWorld);
    server.listen_tcp("localhost:9000");
}
