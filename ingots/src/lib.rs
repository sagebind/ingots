use std::io;
use std::net::SocketAddr;


pub const ENTRYPOINT_SYMBOL: &'static [u8] = b"ingot_entrypoint\0";

/// A static function that produces an ingot instance.
///
/// This type is used when loading ingots from dynamic libraries. Web applications should provide a static `Entrypoint`
/// function named `ingot_entrypoint` that creates an instance of the primary ingot of that crate so they can be loaded
/// dynamically.
///
/// For entry point functions to be reachable, their names must not be mangled, and should always use the `#[no_mangle]`
/// attribute.
pub type Entrypoint = extern fn() -> Box<Ingot>;

/// Primary trait for a Rust ingot. An ingot acts as an entry point for a web application, and provides methods for
/// handling incoming HTTP requests.
///
/// Ingots will be often used in asynchronous or multithreaded contexts, so every ingot is required to be thread-safe
/// and must handle synchronization internally.
pub trait Ingot: Send + Sync {
    fn handle(&self, context: &mut Context);
}

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
