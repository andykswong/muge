//! Scalar type helpers.

use num::Num;

/// Returns negative one of type T.
///
/// # Examples
/// ```
/// assert_eq!(munum::scalar::neg::<f32>(), -1.);
/// ```
#[inline]
pub fn neg<T: Num>() -> T {
    T::zero() - T::one()
}

/// Returns the sign of a.
///
/// # Examples
/// ```
/// assert_eq!(munum::scalar::sign(-2), -1);
/// assert_eq!(munum::scalar::sign(0), 0);
/// assert_eq!(munum::scalar::sign(5), 1);
/// ```
#[inline]
pub fn sign<T: Copy + Num + PartialOrd>(a: T) -> T {
    let zero = T::zero();
    if a < zero {
        neg()
    } else if a == zero {
        zero
    } else {
        T::one()
    }
}

/// Composes a number from the magnitude of a and the sign of b,
/// i.e. copysign(a, b) = sgn(b)|a|.
///
/// # Examples
/// ```
/// assert_eq!(munum::scalar::copysign(-2, 0), 0);
/// assert_eq!(munum::scalar::copysign(3, 4), 3);
/// assert_eq!(munum::scalar::copysign(4, -5), -4);
/// assert_eq!(munum::scalar::copysign(-3, 4), 3);
/// assert_eq!(munum::scalar::copysign(-4, -5), -4);
/// ```
#[inline]
pub fn copysign<T: Copy + Num + PartialOrd>(a: T, b: T) -> T {
    sign(b) * (if a < T::zero() { a * neg() } else { a })
}

/// Linear interpolates between 2 numbers.
///
/// # Examples
/// ```
/// assert_eq!(munum::scalar::lerp(3., 5., 0.5), 4.);
/// ```
#[inline]
pub fn lerp<T: Copy + Num>(a: T, b: T, t: T) -> T {
    a - a * t + b * t
}
