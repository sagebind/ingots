//! Loading of ingot objects at runtime using dynamic linking.
extern crate ingots;
extern crate libloading;
#[macro_use]
extern crate log;

use ingots::*;
use libloading::{Library, Symbol};
use std::ops::{Deref, DerefMut};
use std::path::*;



#[derive(Clone, Copy, Debug)]
pub enum Error {
    LoadLibraryError,
    VersionMismatch,
    UndefinedSymbol,
}


/// Wrapper around an ingot loaded dynamically at runtime.
pub struct DynamicIngot {
    path: PathBuf,
    library: Library,
    ptr: *mut Ingot,
}

impl DynamicIngot {
    /// Open a dynamic ingot from a shared library file.
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<Self, Error> {
        let path = path.into();
        let library = Self::load_library(&path)?;

        // Initialize the ingot instance.
        let ptr = unsafe {
            let __ingot_init: Symbol<extern fn() -> *mut Ingot> = match library.get(b"__ingot_init\0") {
                Ok(v) => v,
                Err(_) => return Err(Error::UndefinedSymbol),
            };

            __ingot_init()
        };

        Ok(Self {
            path: path.into(),
            library: library,
            ptr: ptr,
        })
    }

    /// Get the file system path of the ingot client.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Reload the ingot from the file system.
    pub fn reload(&mut self) -> Result<(), Error> {
        match Self::open(self.path.clone()) {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Load a shared library object.
    fn load_library(path: &Path) -> Result<Library, Error> {
        let library = match Library::new(path) {
            Ok(v) => v,
            Err(_) => return Err(Error::LoadLibraryError),
        };

        // Sanity check: verify ingot API is compatible.
        let library_version = unsafe {
            let symbol: Symbol<*mut u16> = library.get(b"INGOTS_VERSION\0").expect("error loading ingot ABI version");
            **symbol
        };

        debug!("shared library has ingots version: {}", library_version);

        if library_version != INGOTS_VERSION {
            return Err(Error::VersionMismatch);
        }

        Ok(library)
    }
}

impl Drop for DynamicIngot {
    fn drop(&mut self) {
        // Drop the instance using __ingot_free().
        unsafe {
            if let Ok(__ingot_free) = self.library.get::<extern fn(*mut Ingot)>(b"__ingot_free\0") {
                __ingot_free(self.ptr);
            } else {
                warn!("symbol missing: __ingot_free");
                warn!("leaking memory");
            }
        }
    }
}

impl Deref for DynamicIngot {
    type Target = Ingot + 'static;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl DerefMut for DynamicIngot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.ptr
        }
    }
}

unsafe impl Send for DynamicIngot {}
unsafe impl Sync for DynamicIngot {}
