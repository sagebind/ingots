#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
mod abi;
pub mod http;


/// Get the version of the ingots specification this library conforms to.
#[no_mangle]
pub static INGOTS_VERSION: u16 = 1;


/// Primary trait for a Rust ingot. An ingot acts as an entry point for a web application, and provides methods for
/// handling incoming HTTP requests.
///
/// Ingots will be often used in asynchronous or multithreaded contexts, so every ingot is required to be thread-safe
/// and must handle synchronization internally.
pub trait Ingot: Send + Sync {
    /// Handle a single HTTP request.
    fn handle(&self, context: &mut http::Context);

    /// Called by the ingot server when the ingot is put into service.
    fn start(&mut self) {}

    /// Called by the ingot server when the ingot is shut down.
    fn stop(&mut self) {}
}
