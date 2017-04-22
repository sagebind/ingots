use hyper::net::Streaming;
use hyper::server::*;
use ingots;
use std::io;
use std::net::SocketAddr;


pub struct ServerContext<'a, 'b: 'a> {
    server_addr: SocketAddr,
    request: ServerRequest<'a, 'b>,
    response: ServerResponse<'a>,
}

impl<'a, 'b: 'a> ServerContext<'a, 'b> {
    pub fn new(server_addr: SocketAddr, request: Request<'a, 'b>, response: Response<'a>) -> ServerContext<'a, 'b> {
        Self {
            server_addr: server_addr,
            request: ServerRequest(request),
            response: ServerResponse(response.start().unwrap()),
        }
    }
}

impl<'a, 'b: 'a> ingots::Context for ServerContext<'a, 'b> {
    fn remote_addr(&self) -> SocketAddr {
        self.request.0.remote_addr.clone()
    }

    fn server_addr(&self) -> SocketAddr {
        self.server_addr.clone()
    }

    fn server_name(&self) -> String {
        self.server_addr.ip().to_string()
    }

    fn request(&self) -> &ingots::Request {
        &self.request
    }

    fn response(&mut self) -> &mut ingots::Response {
        &mut self.response
    }
}

struct ServerRequest<'a, 'b: 'a>(Request<'a, 'b>);

impl<'a, 'b> ingots::Request for ServerRequest<'a, 'b> {
    fn uri(&self) -> String {
        self.0.uri.to_string()
    }

    fn method(&self) -> String {
        self.0.method.to_string()
    }

    fn protocol(&self) -> String {
        self.0.version.to_string()
    }

    fn query_string(&self) -> Option<String> {
        None
    }

    fn is_secure(&self) -> bool {
        false
    }
}

impl<'a, 'b> io::Read for ServerRequest<'a, 'b> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

struct ServerResponse<'a>(Response<'a, Streaming>);

impl<'a> ingots::Response for ServerResponse<'a> {}

impl<'a> io::Write for ServerResponse<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
