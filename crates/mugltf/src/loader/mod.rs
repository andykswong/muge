mod loader;

pub use loader::*;

#[cfg(feature = "file-loader")]
pub mod file_loader;
#[cfg(feature = "file-loader")]
pub use file_loader::GltfResourceFileLoader;

#[cfg(feature = "fetch-loader")]
pub mod fetch_loader;
#[cfg(feature = "fetch-loader")]
pub use fetch_loader::GltfResourceFetchLoader;
