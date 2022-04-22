//! Marker traits.

use core::fmt::Debug;
use core::hash::Hash;

use num::{NumCast, Unsigned};

/// Marker trait for an unsigned number type castable to/from unsigned primitives
pub trait UnsignedNum: Copy + Debug + Hash + NumCast + Ord + Unsigned {}

impl<U> UnsignedNum for U where U: Copy + Debug + Hash + NumCast + Ord + Unsigned {}
