//! Minimal glTF 2.0 asset loader

#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod asset;
mod error;
mod loader;
pub mod model;

pub use asset::*;
pub use error::*;
pub use loader::*;
pub use model::*;
