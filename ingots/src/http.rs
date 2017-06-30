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

    /// Get the request headers.
    fn headers(&self);

    fn get_header(&self, name: &str) -> Option<&str>;

    /// Check if this is a secure HTTPS connection.
    fn is_secure(&self) -> bool {
        false
    }

    /// Check if this request was made by JavaScript.
    ///
    /// This is typcially determined by checking the value of the `X-Requested-With` header.
    fn is_ajax(&self) -> bool {
        self.get_header("X-Requested-With") == Some("XMLHttpRequest")
    }
}

/// An outgoing HTTP response.
pub trait Response: io::Write {
    /// Get the response status code.
    fn get_status(&self) -> u8;

    /// Set the response status code.
    fn set_status(&mut self, status: u8);

    fn headers(&self);
    fn get_header(&self, name: &str, value: &str);
    fn set_header(&self, name: &str, value: &str);

    /// Check if the response headers have already been sent.
    fn headers_sent(&self) -> bool;

    /// Send the request headers if the have not yet been sent.
    ///
    /// This is done implicitly when the body is first written to.
    fn send_headers(&mut self);

    /// Check if buffering is currently enabled for the response body.
    ///
    /// When output buffering is enabled, data written to the response body is collected in a buffer before being sent
    /// to the client. When disabled, the body is streamed to the client as it written.
    ///
    /// Note that this only indicates if the response is buffered between the web server and the client. Even if
    /// buffering is disabled, there may be additional layers between the server and the user that may perform response
    /// buffering, such as proxies.
    fn get_buffering(&self) -> bool;

    /// Request for output buffering to be enabled or disabled, returning the new buffering policy.
    ///
    /// This method does not explicitly control buffering; it is up to the web server whether responses are buffered or
    /// not. The return value of this method is provided by the web server to indicate if changing the buffering policy
    /// is honored.
    ///
    /// This method shall have no effect if the response body has been written partially or completely, or if headers
    /// have already been written.
    fn set_buffering(&mut self, buffering: bool) -> bool;

    /// Finalize the response and send it to the client.
    fn end(&mut self);
}

pub struct StatusCode(pub u16);

impl StatusCode {
    // pub const OK: Self = StatusCode(200);
}

pub struct BasicRequest {
    status: u8,
}


pub enum Entity {
    Buffered(String),
}
