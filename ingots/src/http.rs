//! Ingot's HTTP interface.
//!
//! This HTTP module is not meant to be full-featured. The API was designed to have as little surface area as possible
//! while still being idiomatic and easy to use. This helps reduce the amount of work required for both servers and
//! handler frameworks to implement and use the interface.
use std::io;
use std::net::SocketAddr;


pub type StatusCode = u16;

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
    /// Get the HTTP request method.
    ///
    /// The string returned is not required to follow strict casing. You should normalize the returned string before
    /// checking for specific HTTP methods.
    fn method(&self) -> String;

    /// Get the request URI.
    ///
    /// The returned URI must start with "/" and must exclude the query string portion of the URI and the "?" separator.
    /// If the request URI cannot be determined, this method should simply return "/".
    fn uri(&self) -> String;

    /// The protocol the request was made with, such as "HTTP/1.1".
    fn protocol(&self) -> String;

    /// Get the query string contained in the request URI, if present.
    fn query_string(&self) -> Option<String> {
        None
    }

    /// Get the request headers.
    ///
    /// Returns an array of HTTP request headers as pairs of the header name and value. Header names are not required to
    /// have the same letter casing as sent by the client.
    fn headers(&self) -> &[(&str, &str)];

    /// Get the value of a request header.
    ///
    /// The header name is case-insensitive, and may not match the exact casing as sent by the client.
    ///
    /// Implementors should override this method if possible, as the default implementation does not perform very well.
    fn get_header(&self, name: &str) -> Option<&str> {
        let name = name.to_lowercase();

        for &(header_name, value) in self.headers() {
            if header_name.to_lowercase() == name {
                return Some(value);
            }
        }

        None
    }

    /// Check if this is a secure HTTPS connection.
    fn is_secure(&self) -> bool {
        false
    }
}

/// An outgoing HTTP response.
///
/// Response objects are not constructed by the application; they are constructed by the web server that is proxying the
/// response. The response is a "write-oriented" API, where content is written to the response sequentially.
pub trait Response: io::Write {
    /// Get the response status code.
    fn status(&self) -> StatusCode;

    /// Set the response status code.
    fn set_status(&mut self, status: StatusCode);

    /// Set the value of the specified header.
    ///
    /// This method will replace any existing headers for the specified field.
    fn set_header(&mut self, name: &str, value: String);

    /// Check if buffering is currently enabled for the response body.
    ///
    /// When output buffering is enabled, data written to the response body is collected in a buffer before being sent
    /// to the client. When disabled, the body is streamed to the client as it written.
    ///
    /// Note that this only indicates if the response is buffered between the web server and the client. Even if
    /// buffering is disabled, there may be additional layers between the server and the user that may perform response
    /// buffering, such as proxies.
    ///
    /// Servers are not required to implement all modes of response buffering, but they *should* return an accurate
    /// description of how the server will handle the response body.
    fn buffering(&self) -> Buffering;

    /// Request for output buffering to be enabled or disabled, returning the new buffering policy.
    ///
    /// This method does not explicitly control buffering; it is up to the web server whether responses are buffered or
    /// not. The return value of this method is provided by the web server to indicate if the given buffering policy
    /// will be honored. Servers are not required to support changing the buffering at all.
    ///
    /// The buffering policy API does not offer as much control to the application as more explicit APIs do, but some
    /// servers may not support all of the ways of handling a response body.
    ///
    /// This method *may* have no effect if the response body has been written partially or completely, or if headers
    /// have already been written.
    fn set_buffering(&mut self, buffering: bool) -> bool;

    /// Check if the response headers have already been sent.
    fn headers_sent(&self) -> bool;
}

/// Defines a policy for buffering the content of a response.
#[derive(Clone, Copy, Debug)]
pub enum Buffering {
    /// Content buffering is enabled with a maximum buffer size (in bytes).
    On(u32),

    /// Content buffering is disabled.
    ///
    /// Unless the `Content-Length` header is explicitly set, the web server *should* stream content written to the
    /// response to the browser. For HTTP/1.1, the web server may use chunked transfer encoding; in HTTP/2, content
    /// might be written to the output stream directly.
    Off,
}
