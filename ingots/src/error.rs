#[derive(Debug)]
pub enum Error {
    EntrypointNotFound,
    LoadLibraryError,
    VersionMismatch,
}
