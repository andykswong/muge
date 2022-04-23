//! Minimalistic Data Structures and Entity-Component-System Library.

#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod collections;
pub mod ecs;

mod genindex;
mod markers;

pub use genindex::*;
pub use markers::*;

/// Commonly used types.
pub mod prelude {
    pub use super::collections::traits::*;
    pub use super::ecs::prelude::*;
    pub use super::genindex::GenIndex;
    pub use super::markers::*;
    pub use crate::{cons, Cons};
}
