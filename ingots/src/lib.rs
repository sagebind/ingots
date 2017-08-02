#![allow(dead_code)]
#![allow(unused_variables)]
pub extern crate http;

#[macro_use]
mod abi;

use std::borrow::Cow;
use std::io;
use std::net::SocketAddr;


/// Get the version of the ingots specification this library conforms to.
#[no_mangle]
pub static INGOTS_VERSION: u16 = 1;


/// Primary trait for a Rust ingot. An ingot acts as an entry point for a web application, and provides methods for
/// handling incoming HTTP requests.
///
/// This is the part of the interface that applications and frameworks should implement.
///
/// Ingots will be often used in asynchronous or multithreaded contexts, so every ingot is required to be thread-safe
/// and must handle synchronization internally.
pub trait Ingot: Send + Sync {
    /// Handle a single HTTP request.
    fn handle(&self, context: &mut Context);

    /// Called by the ingot server when the ingot is put into service.
    fn start(&mut self) {}

    /// Called by the ingot server when the ingot is stopped.
    ///
    /// This is distinct from implementing a drop handler, since an ingot may be started and stopped multiple times
    /// during the lifetime of the instance.
    fn stop(&mut self) {}
}

impl<F> Ingot for F where F: Fn(&mut Context) + Send + Sync {
    fn handle(&self, context: &mut Context) {
        (self)(context)
    }
}


/// Encapsulates the state of an individual HTTP request from the web server.
///
/// This is the part of the interface that web servers and gateways should implement.
pub trait Context: Send {
    /// Get the address of the remote client.
    fn remote_addr(&self) -> SocketAddr;

    /// Get the address of the server.
    fn server_addr(&self) -> SocketAddr;

    /// Get the name of the server.
    fn server_name(&self) -> Cow<str>;

    /// Get the portion of the URI path that corresponds to this application object.
    ///
    /// This part of the path **must not** appear in the request URI.
    fn context_path(&self) -> Cow<str>;

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
    fn set_buffering(&mut self, buffering: Buffering) -> Buffering;

    /// Get the HTTP request for the current request.
    fn request(&self) -> &mut ServerRequest;

    /// Get the HTTP response for the current request.
    fn response(&mut self) -> &mut ServerResponse;
}


pub type ServerRequest = http::Request<Box<io::Read>>;
pub type ServerResponse = http::Response<Box<io::Write>>;


/// Defines a policy for buffering the content of a response.
#[derive(Clone, Copy, Debug)]
pub enum Buffering {
    /// Content buffering is enabled with a maximum buffer size (in bytes).
    On(u32),

    /// Content buffering is disabled.
    ///
    /// Unless the `Content-Length` header is explicitly set, the web server **should** stream content written to the
    /// response to the browser. For HTTP/1.1, the web server may use chunked transfer encoding; in HTTP/2, content
    /// might be written to the output stream directly.
    Off,
}
