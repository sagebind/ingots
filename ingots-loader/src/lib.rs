#![allow(dead_code)]
extern crate ingots;
extern crate libloading;
#[macro_use]
extern crate log;

use ingots::*;
use libloading::{Library, Symbol};
use std::path::PathBuf;


/// Wrapper around an ingot loaded dynamically at runtime.
pub struct DynamicIngot {
    path: PathBuf,
    library: Option<Library>,
    ingot: Option<Box<Ingot>>,
}

impl DynamicIngot {
    /// Open a dynamic ingot from a shared library file.
    pub fn open<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            library: None,
            ingot: None,
        }
    }

    /// Check if the ingot is currently loaded.
    pub fn is_loaded(&self) -> bool {
        self.ingot.is_some()
    }

    /// Reload the ingot from the file system.
    pub fn reload(&mut self) -> Result<(), ()> {
        // Open the shared library.
        let library = match Library::new(&self.path) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        {
            /// Find the symbol for the ingot entrypoint.
            let entrypoint: Symbol<Entrypoint> = match unsafe {
                library.get(ENTRYPOINT_SYMBOL)
            } {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

            // Invoke the entrypoint function to create the ingot instance.
            self.ingot = Some(entrypoint());
        }

        self.library = Some(library);

        Ok(())
    }
}

impl Ingot for DynamicIngot {
    fn handle(&self, context: &mut Context) {
        if let Some(ref ingot) = self.ingot {
            ingot.handle(context);
        } else {
            warn!("ingot not loaded, dropping request: {:?}", self.path);
        }
    }
}

impl Drop for DynamicIngot {
    fn drop(&mut self) {
        // Make sure the ingot instance is dropped before the ingot library.
        drop(self.ingot.take());
        drop(self.library.take());
    }
}
