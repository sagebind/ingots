#![allow(dead_code)]
#[macro_use]
extern crate log;

#[cfg(feature = "dynamic")]
pub mod dynamic;
pub mod http;
mod error;

pub use error::*;
use std::ops::{Deref, DerefMut};


/// Get the version of the ingots specification this library conforms to.
#[no_mangle]
pub fn get_ingots_version() -> u16 {
    1
}

/// Primary trait for a Rust ingot. An ingot acts as an entry point for a web application, and provides methods for
/// handling incoming HTTP requests.
///
/// Ingots will be often used in asynchronous or multithreaded contexts, so every ingot is required to be thread-safe
/// and must handle synchronization internally.
pub trait Ingot: Send + Sync {
    fn handle(&self, context: &mut http::Context);
}

/// Owned pointer around an ingot object that is safe to share across binary boundaries.
///
/// Unlike a `Box`, this carries with it a pointer to a deallocation function. This ensures that the binary that
/// originally allocated the value will also deallocate the value, even when dropped in a different binary.
pub struct IngotBox {
    /// Raw pointer to the inner value.
    ptr: *mut Ingot,

    /// Pointer to the deallocator function.
    free: fn(*mut Ingot),
}

impl<T: Ingot + 'static> From<T> for IngotBox {
    fn from(ingot: T) -> IngotBox {
        fn free(ptr: *mut Ingot) {
            unsafe {
                Box::from_raw(ptr);
            }
        }

        IngotBox {
            ptr: Box::into_raw(Box::new(ingot)),
            // This is what carries the current binary's implementation of `free` around with the value.
            free: free,
        }
    }
}

impl Drop for IngotBox {
    fn drop(&mut self) {
        (self.free)(self.ptr);
    }
}

impl Deref for IngotBox {
    type Target = Ingot + 'static;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl DerefMut for IngotBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.ptr
        }
    }
}

unsafe impl Send for IngotBox {}
unsafe impl Sync for IngotBox {}
