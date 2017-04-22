//! Loading of ingot objects at runtime using dynamic linking.
extern crate libloading;

use super::*;
use self::libloading::{Library, Symbol};
use std::path::*;


/// The default symbol name for an ingot entry point function.
pub const DEFAULT_ENTRYPOINT: &'static str = "ingot_entrypoint";

/// A static function that produces an ingot instance.
///
/// This type is used when loading ingots from dynamic libraries. Web applications should provide a static `Entrypoint`
/// function named `ingot_entrypoint` that creates an instance of the primary ingot of that crate so they can be loaded
/// dynamically.
///
/// For entry point functions to be reachable, their names must not be mangled, and should always use the `#[no_mangle]`
/// attribute.
pub type Entrypoint = fn() -> IngotBox;


/// Wrapper around an ingot loaded dynamically at runtime.
pub struct DynamicIngot {
    path: PathBuf,
    symbol: Vec<u8>,
    library: Option<Library>,
    ingot: Option<IngotBox>,
}

impl DynamicIngot {
    /// Open a dynamic ingot from a shared library file.
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<Self, Error> {
        Self::open_symbol(path, DEFAULT_ENTRYPOINT)
    }

    /// Open a dynamic ingot from a shared library file with a specific symbol name.
    pub fn open_symbol<P: Into<PathBuf>, S: AsRef<[u8]>>(path: P, name: S) -> Result<Self, Error> {
        let mut symbol = name.as_ref().to_owned();
        symbol.push(0);

        let mut ingot = Self {
            path: path.into(),
            symbol: symbol,
            library: None,
            ingot: None,
        };

        // Attempt to load the ingot before we return successfully.
        match ingot.reload() {
            Ok(_) => Ok(ingot),
            Err(e) => Err(e),
        }
    }

    /// Get the file system path of the ingot client.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if the ingot is currently loaded.
    pub fn is_loaded(&self) -> bool {
        self.ingot.is_some()
    }

    /// Reload the ingot from the file system.
    pub fn reload(&mut self) -> Result<(), Error> {
        // Unload the previous instance, if any.
        self.unload();

        // Open the shared library.
        let library = match Library::new(&self.path) {
            Ok(v) => v,
            Err(_) => return Err(Error::LoadLibraryError),
        };

        // Sanity check: verify ingot API is compatible.
        if !Self::library_version_matches(&library, get_ingots_version()) {
            return Err(Error::VersionMismatch);
        }

        {
            /// Find the symbol for the ingot entrypoint.
            let entrypoint: Symbol<Entrypoint> = match unsafe {
                library.get(&self.symbol)
            } {
                Ok(v) => v,
                Err(_) => return Err(Error::EntrypointNotFound),
            };

            // Invoke the entrypoint function to create the ingot instance.
            self.ingot = Some(entrypoint());
        }

        self.library = Some(library);

        Ok(())
    }

    /// Unload the ingot instance.
    pub fn unload(&mut self) {
        // Make sure the ingot instance is dropped before the ingot library.
        drop(self.ingot.take());
        drop(self.library.take());
    }

    /// Verify that the ingots version of a library matches the given version.
    fn library_version_matches(library: &Library, version: u16) -> bool {
        let version_fn: Symbol<fn() -> u16> = match unsafe {
            library.get(b"get_ingots_version\0")
        } {
            Ok(v) => v,
            Err(_) => return false,
        };

        let library_version = version_fn();
        debug!("shared library has ingots version: {}", library_version);

        library_version == version
    }
}

impl Ingot for DynamicIngot {
    fn handle(&self, context: &mut http::Context) {
        if let Some(ref ingot) = self.ingot {
            ingot.handle(context);
        } else {
            warn!("ingot not loaded, dropping request: {:?}", self.path);
        }
    }
}

impl Drop for DynamicIngot {
    fn drop(&mut self) {
        self.unload();
    }
}
