#![allow(dead_code)]
pub mod http;
mod error;

pub use error::*;
use std::ops::{Deref, DerefMut};


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

/// Owned pointer around an ingot object that is safe to share across binary boundaries.
///
/// Unlike a `Box`, this carries with it a pointer to a deallocation function. This ensures that the binary that
/// originally allocated the value will also deallocate the value, even when dropped in a different binary.
pub struct IngotRef {
    /// Raw pointer to the inner value.
    ptr: *mut Ingot,

    /// Pointer to the deallocator function.
    free: fn(*mut Ingot),
}

impl<T: Ingot + 'static> From<T> for IngotRef {
    fn from(ingot: T) -> IngotRef {
        fn free(ptr: *mut Ingot) {
            unsafe {
                Box::from_raw(ptr);
            }
        }

        IngotRef {
            ptr: Box::into_raw(Box::new(ingot)),
            // This is what carries the current binary's implementation of `free` around with the value.
            free: free,
        }
    }
}

impl Drop for IngotRef {
    fn drop(&mut self) {
        (self.free)(self.ptr);
    }
}

impl Deref for IngotRef {
    type Target = Ingot + 'static;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl DerefMut for IngotRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.ptr
        }
    }
}

unsafe impl Send for IngotRef {}
unsafe impl Sync for IngotRef {}
