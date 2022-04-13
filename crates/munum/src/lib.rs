#![no_std]

#[cfg(test)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod matrix;
mod matrix_special;
mod matrix_ops;
mod quat;

pub mod float_eq;
pub mod scalar;
pub mod transform;

pub use float_eq::FloatEq;
pub use matrix::Matrix;
pub use matrix_special::*;
pub use quat::{quat, Quaternion};
