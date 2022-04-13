mod app;
mod entry;
mod model;

pub use app::*;
pub use entry::*;
pub use model::*;

#[cfg(all(target_family = "wasm", feature = "backend-webgl"))]
pub const APP_ID: mugl::webgl::ContextId = mugl::webgl::ContextId::new(1.);
