//! Minimalistic Low-level WebGL 2.0 / WebGPU 3D graphics abstraction layer for Rust and WebAssembly

#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod gpu;
pub mod alias;
pub mod descriptor;
pub mod primitive;
pub mod gl_const;

pub use alias::*;
pub use descriptor::*;
pub use primitive::*;
pub use gpu::*;

/// Core types.
pub mod prelude {
    pub use crate::alias::*;
    pub use crate::descriptor::*;
    pub use crate::primitive::*;
    pub use crate::gpu::*;
}

// GPU backends

pub mod empty;

#[cfg(feature = "backend-wgpu")]
pub mod wgpu;

#[cfg(feature = "backend-webgl")]
pub mod webgl;
