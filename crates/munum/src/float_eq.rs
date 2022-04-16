//! Float type comparison helpers.

use num::traits::{float::FloatCore, NumAssign, NumCast};
use crate::{Matrix, Quaternion};

/// Standard tolerance epsilon
pub const EPSILON: f32 = 128. * core::f32::EPSILON;

/// Returns a standard tolerance epsilon
pub fn epsilon<T: NumCast>() -> T {
    NumCast::from(EPSILON).expect("incompatible type")
}

/// Trait for checking if 2 floats or float arrays are equal within an epsilon.
pub trait FloatEq<T: Copy> {
    /// Checks if self equals to RHS within an epsilon.
    /// # Examples
    /// ```
    /// # use munum::FloatEq;
    /// assert!(1_f32.float_eq(1.000001, 0.0001));
    /// assert!(!1_f32.float_eq(1.1, 0.0001));
    /// ```
    fn float_eq(&self, rhs: Self, epsilon: T) -> bool;
}

impl FloatEq<f32> for f32 {
    fn float_eq(&self, rhs: Self, epsilon: f32) -> bool {
        FloatCore::abs(*self - rhs) < epsilon
    }
}

impl FloatEq<f64> for f64 {
    fn float_eq(&self, rhs: Self, epsilon: f64) -> bool {
        FloatCore::abs(*self - rhs) < epsilon
    }
}

impl<T: Copy, V: Copy + FloatEq<T>> FloatEq<T> for &[V] {
    fn float_eq(&self, rhs: Self, epsilon: T) -> bool {
        if self.len() != rhs.len() {
            return false;
        }

        for i in 0..self.len() {
            if !self[i].float_eq(rhs[i], epsilon) {
                return false;
            }
        }

        true
    }
}

impl<T: Copy + FloatEq<T> + NumAssign, const R: usize, const C: usize> FloatEq<T>
    for Matrix<T, R, C>
{
    #[inline]
    fn float_eq(&self, rhs: Self, epsilon: T) -> bool {
        self.as_ref().float_eq(rhs.as_ref(), epsilon)
    }
}

impl<T: Copy + FloatEq<T> + NumAssign> FloatEq<T> for Quaternion<T> {
    #[inline]
    fn float_eq(&self, rhs: Self, epsilon: T) -> bool {
        self.as_ref().float_eq(rhs.as_ref(), epsilon)
    }
}

/// Asserts two floats or float arrays are equal within an epsilon.
/// # Examples
/// ```
/// # use munum::assert_float_eq;
/// assert_float_eq!(1.0, 1.000001, 0.0001);
/// ```
#[macro_export]
macro_rules! assert_float_eq {
    ($left:expr, $right:expr) => {
        assert_float_eq!(($left), ($right), $crate::float_eq::epsilon())
    };

    ($left:expr, $right:expr, $epsilon:expr) => {
        match (&($left), &($right), ($epsilon)) {
            (left_val, right_val, epsilon_val) => {
                if !$crate::FloatEq::float_eq(left_val, *right_val, epsilon_val) {
                    panic!(
                        "assertion failed: `left.float_eq(right, epsilon)` \
                                (left: `{:?}`, right: `{:?}`, epsilon: `{:?}`)",
                        *left_val, *right_val, epsilon_val
                    );
                }
            }
        }
    };
}
