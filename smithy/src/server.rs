use config::*;
use context::ServerContext;
use engine::IngotEngine;
use ingots::Ingot;
use hyper::server::Server as HttpServer;
use hyper::server::Request as HttpRequest;
use hyper::server::Response as HttpResponse;
use hyper::server::Handler as HttpHandler;
use std::net::SocketAddr;


pub struct Server {
    config: ServerConfig,
    server: Option<HttpServer>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config: config,
            server: Some(HttpServer::http("0.0.0.0:8001").unwrap()),
        }
    }

    pub fn listen(&mut self) {
        let local_addr = self.server.as_mut().unwrap().local_addr().unwrap();

        let listener = self.server.take()
            .unwrap()
            .handle(Handler::new(local_addr, self.config.clone()))
            .unwrap();
    }

}

struct Handler {
    config: ServerConfig,
    engine: IngotEngine,
    local_addr: SocketAddr,
}

impl Handler {
    fn new(local_addr: SocketAddr, config: ServerConfig) -> Handler {
        let mut engine = IngotEngine::new();

        for location in config.locations.iter() {
            engine.register(location.prefix.clone(), &location.ingot);
        }

        Handler {
            config: config,
            engine: engine,
            local_addr: local_addr,
        }
    }
}

impl HttpHandler for Handler {
    fn handle<'a, 'b>(&'a self, request: HttpRequest<'a, 'b>, response: HttpResponse<'a>) {
        info!("{} {}", request.method, request.uri);

        let url = request.uri.to_string();
        let mut context = ServerContext::new(self.local_addr.clone(), request, response);

        if let Some(ingot) = self.engine.find_ingot_for_url(&url) {
            ingot.handle(&mut context);
        }
    }
}
