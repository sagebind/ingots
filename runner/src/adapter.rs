use ingots::http;
use std::io;
use std::net::SocketAddr;
use tiny_http;


pub struct Context {
    server_addr: SocketAddr,
    request: Request,
    response: Response,
}

impl http::Context for Context {
    fn remote_addr(&self) -> SocketAddr {
        // TODO
        self.server_addr.clone()
    }

    fn server_addr(&self) -> SocketAddr {
        self.server_addr.clone()
    }

    fn server_name(&self) -> String {
        format!("{}", self.server_addr.ip())
    }

    fn request(&self) -> &http::Request {
        &self.request
    }

    fn response(&mut self) -> &mut http::Response {
        &mut self.response
    }
}


struct Request(tiny_http::Request);

impl http::Request for Request {
    fn method(&self) -> String {
        format!("{}", self.0.method())
    }

    fn uri(&self) -> String {
        self.0.url().to_owned()
    }

    fn protocol(&self) -> String {
        format!("{}", self.0.http_version())
    }

    fn headers(&self) -> &[(&str, &str)] {
        &[]
    }
}

impl io::Read for Request {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}


struct Response {
    status: http::StatusCode,
    content: io::Cursor<Vec<u8>>,
}

impl http::Response for Response {
    fn status(&self) -> http::StatusCode {
        self.status
    }

    fn set_status(&mut self status: http::StatusCode) {
        self.status = status;
    }

    fn set_header(&mut self, name: &str, value: String);

    fn buffering(&self) -> http::Buffering {
        Buffering::On(65536)
    }

    fn headers_sent(&self) -> bool;
}

impl io::Write for Response {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // self
    }

    fn flush(&mut self) -> io::Result<()> {

    }
}
