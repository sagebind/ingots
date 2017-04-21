use fastcgi;
use ingots;
use std::io;
use std::net::{ToSocketAddrs, SocketAddr};


pub struct Context {
    fastcgi_request: fastcgi::Request,
    remote_addr: SocketAddr,
    server_addr: SocketAddr,
    server_name: String,
    query_string: Option<String>,
}

impl From<fastcgi::Request> for Context {
    fn from(request: fastcgi::Request) -> Context {
        let remote_addr = request.param("REMOTE_ADDR").unwrap();
        let remote_port: u16 = request.param("REMOTE_PORT").unwrap().parse().unwrap();
        let remote_addr = (remote_addr.as_str(), remote_port).to_socket_addrs().unwrap().next().unwrap();

        let server_addr = request.param("SERVER_ADDR").unwrap();
        let server_port: u16 = request.param("SERVER_PORT").unwrap().parse().unwrap();
        let server_addr = (server_addr.as_str(), server_port).to_socket_addrs().unwrap().next().unwrap();

        let server_name = request.param("SERVER_NAME").unwrap_or(String::new());

        let query_string = request.param("REQUEST_URI").and_then(|uri| {
            if let Some(index) = uri.find('?') {
                Some(uri[index+1..].to_string())
            } else {
                None
            }
        });

        Context {
            fastcgi_request: request,
            remote_addr: remote_addr,
            server_addr: server_addr,
            server_name: server_name,
            query_string: query_string,
        }
    }
}

impl ingots::Context for Context {
    fn remote_addr(&self) -> SocketAddr {
        self.remote_addr.clone()
    }

    fn server_addr(&self) -> SocketAddr {
        self.server_addr.clone()
    }

    fn server_name(&self) -> String {
        self.server_name.clone()
    }

    fn request(&self) -> &ingots::Request {
        self
    }

    fn response(&mut self) -> &mut ingots::Response {
        self
    }
}

impl ingots::Request for Context {
    fn uri(&self) -> String {
        self.fastcgi_request.param("REQUEST_URI").unwrap_or(String::from("/"))
    }

    fn method(&self) -> String {
        self.fastcgi_request.param("REQUEST_METHOD").unwrap_or(String::from("GET"))
    }

    fn protocol(&self) -> String {
        self.fastcgi_request.param("SERVER_PROTOCOL").unwrap()
    }

    fn query_string(&self) -> Option<String> {
        self.query_string.clone()
    }

    fn is_secure(&self) -> bool {
        self.fastcgi_request.param("REQUEST_SCHEME")
            .map(|scheme| scheme.to_lowercase() == "https")
            .unwrap_or(false)
    }
}

impl io::Read for Context {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.fastcgi_request.stdin().read(buf)
    }
}

impl ingots::Response for Context {}

impl io::Write for Context {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.fastcgi_request.stdout().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.fastcgi_request.stdout().flush()
    }
}
