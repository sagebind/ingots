use std::io;
use std::net::SocketAddr;


/// Encapsulates the state of an individual HTTP request from the web server.
pub trait Context {
    /// Get the address of the remote client.
    fn remote_addr(&self) -> SocketAddr;

    /// Get the address of the server.
    fn server_addr(&self) -> SocketAddr;

    /// Get the name of the server.
    fn server_name(&self) -> String;

    /// Get the HTTP request for the current request.
    fn request(&self) -> &Request;

    /// Get the HTTP response for the current request.
    fn response(&mut self) -> &mut Response;
}

/// An incoming HTTP request.
///
/// Provides information about a client request, including parameters, attributes, and a request body stream.
pub trait Request: io::Read {
    /// Get the request URI.
    ///
    /// The returned URI must start with "/" and must exclude the query string portion of the URI and the "?" separator.
    /// If the request URI cannot be determined, this method should simply return "/".
    fn uri(&self) -> String;

    /// Get the HTTP request method.
    ///
    /// The string returned is not required to follow strict casing. You should normalize the returned string before
    /// checking for specific HTTP methods.
    fn method(&self) -> String;

    /// The protocol the request was made with, such as "HTTP/1.1".
    fn protocol(&self) -> String;

    /// Get the query string contained in the request URI, if present.
    fn query_string(&self) -> Option<String> {
        None
    }

    /// Check if this is a secure HTTPS connection.
    fn is_secure(&self) -> bool {
        false
    }
}

/// An outgoing HTTP response.
pub trait Response: io::Write {
    fn write_header(&mut self, name: &str, value: &str) -> io::Result<()> {
        let header = format!("{}: {}\n", name, value);
        self.write_all(header.as_bytes())
    }
}
