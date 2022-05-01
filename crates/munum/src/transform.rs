//! Transformation matrix functions.

use num::traits::NumAssign;

#[cfg(any(feature = "std", feature = "libm"))]
use num::traits::Float;

use crate::{scalar, Mat3, Mat4, Quaternion, Vec3, Vec4};

// region: Affine transformations

/// Creates a 4x4 transformation matrix that represents a translation of (x, y, z).
///
/// # Examples
/// ```
/// # use munum::{transform, vec3};
/// assert_eq!(*transform::translation(vec3(2_i32, 3, 5)).as_ref(), [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 2, 3, 5, 1]);
/// ```
pub fn translation<T: Copy + NumAssign>(v: Vec3<T>) -> Mat4<T> {
    let mut result = Mat4::identity();
    result[(0, 3)] = v[0];
    result[(1, 3)] = v[1];
    result[(2, 3)] = v[2];
    result
}

/// Creates a 4x4 transformation matrix that represents a scaling of (x, y, z).
///
/// # Examples
/// ```
/// # use munum::{transform, vec3};
/// assert_eq!(*transform::scaling(vec3(2_i32, 3, 5)).as_ref(), [2, 0, 0, 0, 0, 3, 0, 0, 0, 0, 5, 0, 0, 0, 0, 1]);
/// ```
pub fn scaling<T: Copy + NumAssign>(v: Vec3<T>) -> Mat4<T> {
    let mut result = Mat4::identity();
    result[(0, 0)] = v[0];
    result[(1, 1)] = v[1];
    result[(2, 2)] = v[2];
    result
}

/// Creates a 4x4 transformation matrix that represents a rotation by a quaternion.
///
/// # Examples
/// ```
/// # use munum::{transform, quat, assert_float_eq};
/// assert_float_eq!(
///     transform::rotation(quat(1./3_f32.sqrt(), 1./3_f32.sqrt(), 1./3_f32.sqrt(), 0.)).as_ref(),
///     &[-1./3., 2./3., 2./3., 0., 2./3., -1./3., 2./3., 0., 2./3., 2./3., -1./3., 0., 0., 0., 0., 1.]
/// );
/// ```
#[inline]
pub fn rotation<T: Copy + NumAssign>(q: Quaternion<T>) -> Mat4<T> {
    Mat3::from(q).into()
}

/// Creates a 4x4 matrix that represents a transformation in TRS order (= translation * rotation * scaling).
///
/// # Examples
/// ```
/// # use munum::{transform, quat, vec3, assert_float_eq};
/// assert_float_eq!(
///     transform::transformation(
///         vec3(1., 2., 3.),
///         quat(0.5/3_f32.sqrt(), 0.5/3_f32.sqrt(), 0.5/3_f32.sqrt(), 3_f32.sqrt()/2.),
///         vec3(3., 6., 9.),
///     ).as_ref(),
///     &[2., 2., -1., 0., -2., 4., 4., 0., 6., -3., 6., 0., 1., 2., 3., 1.]
/// );
/// ```
pub fn transformation<T: Copy + NumAssign>(
    translation: Vec3<T>,
    rotation: Quaternion<T>,
    scaling: Vec3<T>,
) -> Mat4<T> {
    // Start with rotation
    let mut result: Mat4<T> = Mat3::from(rotation).into();

    // Post-multiply scaling
    for c in 0..3 {
        for r in 0..3 {
            result[(r, c)] *= scaling[c];
        }
    }

    // Apply translation
    result[(0, 3)] = translation[0];
    result[(1, 3)] = translation[1];
    result[(2, 3)] = translation[2];

    result
}

/// Extracts the (x, y, z) translation component from a 4x4 TRS transformation matrix.
///
/// # Examples
/// ```
/// # use munum::{transform, vec3, Mat4};
/// assert_eq!(*transform::translation_of(<Mat4>::from_slice(&[2., 2., -1., 0., -2., 4., 4., 0., 6., -3., 6., 0., 11., 12., 13., 1.])).as_ref(), [11., 12., 13.]);
/// ```
pub fn translation_of<T: Copy + NumAssign>(m: Mat4<T>) -> Vec3<T> {
    Vec3::new([[m[(0, 3)], m[(1, 3)], m[(2, 3)]]])
}

/// Extracts the (x, y, z) scaling component from a 4x4 TRS transformation matrix.
///
/// # Examples
/// ```
/// # use munum::{transform, vec3, Mat4};
/// assert_eq!(*transform::scaling_of(<Mat4>::from_slice(&[2., 2., -1., 0., -2., 4., 4., 0., 6., -3., 6., 0., 11., 12., 13., 1.])).as_ref(), [3., 6., 9.]);
/// ```
#[cfg(any(feature = "std", feature = "libm"))]
#[inline]
pub fn scaling_of<T: Copy + Float + NumAssign>(m: Mat4<T>) -> Vec3<T> {
    Vec3::new([[
        Vec3::new([[m[(0, 0)], m[(1, 0)], m[(2, 0)]]]).len(),
        Vec3::new([[m[(0, 1)], m[(1, 1)], m[(2, 1)]]]).len(),
        Vec3::new([[m[(0, 2)], m[(1, 2)], m[(2, 2)]]]).len(),
    ]])
}

/// Extracts the rotation quaternion component from a 4x4 TRS transformation matrix.
///
/// # Examples
/// ```
/// # use munum::{transform, vec3, Mat4, assert_float_eq};
/// assert_float_eq!(
///     transform::rotation_of(<Mat4>::from_slice(&[2., 2., -1., 0., -2., 4., 4., 0., 6., -3., 6., 0., 11., 12., 13., 1.])).as_ref(),
///     &[0.5/3_f32.sqrt(), 0.5/3_f32.sqrt(), 0.5/3_f32.sqrt(), 3_f32.sqrt()/2.]
/// );
/// ```
#[cfg(any(feature = "std", feature = "libm"))]
pub fn rotation_of<T: Copy + Float + NumAssign>(m: Mat4<T>) -> Quaternion<T> {
    let zero = T::zero();
    let one = T::one();
    let two = one + one;
    let scaling = scaling_of(m);
    let m00 = m[(0, 0)] / scaling[0];
    let m11 = m[(1, 1)] / scaling[1];
    let m22 = m[(2, 2)] / scaling[2];

    Quaternion::from_slice(&[
        scalar::copysign(
            zero.max(one + m00 - m11 - m22).sqrt() / two,
            m[(2, 1)] / scaling[1] - m[(1, 2)] / scaling[2],
        ),
        scalar::copysign(
            zero.max(one - m00 + m11 - m22).sqrt() / two,
            m[(0, 2)] / scaling[2] - m[(2, 0)] / scaling[0],
        ),
        scalar::copysign(
            zero.max(one - m00 - m11 + m22).sqrt() / two,
            m[(1, 0)] / scaling[0] - m[(0, 1)] / scaling[1],
        ),
        zero.max(one + m00 + m11 + m22).sqrt() / two,
    ])
}

/// Inverts a `Mat4` that represents a valid transformation in TRS order (= translation * rotation * scale).
/// This function is more efficient than `Mat4::invert` by leveraging the properties of a TRS matrix.
///
/// # Examples
/// ```
/// # use munum::{transform, Mat4, assert_float_eq};
/// let mut m = <Mat4>::from_slice(&[2., 2., -1., 0., -2., 4., 4., 0., 6., -3., 6., 0., 1., 2., 3., 1.]);
/// transform::invert_trs(&mut m);
/// assert_float_eq!(m.as_ref(), &[2./9., -1./18., 2./27., 0., 2./9., 1./9., -1./27., 0., -1./9., 1./9., 2./27., 0., -1./3., -1./2., -2./9., 1.]);
/// ```
#[cfg(any(feature = "std", feature = "libm"))]
pub fn invert_trs<T: Copy + Float + NumAssign>(m: &mut Mat4<T>) {
    // Assume M is a TRS matrix:
    // M = T * R * S = [RS  t]
    //                 [0   1]
    // Then the inverse of M is:
    // M^-1 = [(RS)^-1  (RS)^-1 * -t]
    //        [   0           1     ]
    // Where: (RS)^-1 = S^-1 * R^-1 = S^-1 * RT = S^-1 * ((RS)(S^-1))T = S^-1 * (S^-1)T * (RS)T = S^-1 * S^-1 * (RS)T

    let zero = T::zero();
    let one = T::one();
    let neg = scalar::neg();

    // Extract S and t
    let scaling = scaling_of(*m);
    let translation = translation_of(*m);

    // Calculate m = (RS)T
    m.transpose();
    m[(3, 0)] = zero;
    m[(3, 1)] = zero;
    m[(3, 2)] = zero;

    // Premultiply S^-2 = 1/(S*S) to m
    for c in 0..3 {
        for r in 0..3 {
            m[(r, c)] *= one / (scaling[r] * scaling[r]);
        }
    }

    // Now m = (RS)^-1
    // Apply translation = (m * -t) to m
    let mut t = Vec4::from_vec3(translation, zero);
    t *= neg;
    t.mul_assign(*m, t);

    m[(0, 3)] = t[0];
    m[(1, 3)] = t[1];
    m[(2, 3)] = t[2];
}

// endregion: Affine transformations

// region: Projection matrices

/// Creates the {@link Mat4} orthographic projection matrix.
/// To apply a glTF orthographic camera, use: left = -xmag, right = xmag, bottom = -ymag, top = ymag.
/// See: <https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html#projection-matrices>
///
/// # Examples
/// ```
/// # use munum::{transform, assert_float_eq};
/// assert_float_eq!(
///     transform::ortho(-1., 3., -7., 4., -2., 5.).as_ref(),
///     &[1./2., 0., 0., 0., 0., 2./11., 0., 0., 0., 0., -2./7., 0., -1./2., 3./11., -3./7., 1.]
/// );
/// ```
pub fn ortho<T: Copy + NumAssign>(
    left: T,
    right: T,
    bottom: T,
    top: T,
    znear: T,
    zfar: T,
) -> Mat4<T> {
    let neg = scalar::neg();
    let one = T::one();
    let two = one + one;
    let x = one / (right - left);
    let y = one / (top - bottom);
    let z = one / (znear - zfar);
    let mut result = Mat4::identity();
    result[(0, 0)] = two * x;
    result[(1, 1)] = two * y;
    result[(2, 2)] = two * z;

    result[(0, 3)] = (right + left) * x * neg;
    result[(1, 3)] = (top + bottom) * y * neg;
    result[(2, 3)] = (znear + zfar) * z;

    result
}

/// Creates the 4x4 perspective projection using glTF's formula.
/// Uses infinite projection if zfar = Infinity.
/// See: <https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html#projection-matrices>
///
/// # Examples
/// ```
/// # use core::f32::consts::PI;
/// # use core::f32::INFINITY;
/// # use munum::{transform, assert_float_eq};
/// assert_float_eq!(
///     transform::perspective(2., PI/2., 1., INFINITY).as_ref(),
///     &[0.5, 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., -1., 0., 0., -2., 0.]
/// );
/// assert_float_eq!(
///     transform::perspective(2., PI/2., 1., 9.).as_ref(),
///     &[0.5, 0., 0., 0., 0., 1., 0., 0., 0., 0., -1.25, -1., 0., 0., -2.25, 0.]
/// );
/// ```
#[cfg(any(feature = "std", feature = "libm"))]
#[inline]
pub fn perspective<T: Copy + Float + NumAssign>(aspect: T, yfov: T, znear: T, zfar: T) -> Mat4<T> {
    let two = T::one() + T::one();
    let top = znear * (yfov / two).tan();
    let right = aspect * top;
    perspective_viewport(-right, right, -top, top, znear, zfar)
}

/// Creates the 4x4 perspective projection from viewport and range.
/// Uses infinite projection if zfar = Infinity.
///
/// # Examples
/// ```
/// # use core::f32::INFINITY;
/// # use munum::{transform, assert_float_eq};
/// assert_float_eq!(
///     transform::perspective_viewport(-1.0, 3.0, -0.5, 1.5, 1., INFINITY).as_ref(),
///     &[0.5, 0., 0., 0., 0., 1., 0., 0., 0.5, 0.5, -1., -1., 0., 0., -2., 0.]
/// );
/// assert_float_eq!(
///     transform::perspective_viewport(-1.0, 3.0, -0.5, 1.5, 1., 9.).as_ref(),
///     &[0.5, 0., 0., 0., 0., 1., 0., 0., 0.5, 0.5, -1.25, -1., 0., 0., -2.25, 0.]
/// );
/// ```
#[cfg(any(feature = "std", feature = "libm"))]
pub fn perspective_viewport<T: Copy + Float + NumAssign>(
    left: T,
    right: T,
    bottom: T,
    top: T,
    znear: T,
    zfar: T,
) -> Mat4<T> {
    let one = T::one();
    let two = one + one;

    let x = one / (right - left);
    let y = one / (top - bottom);

    let mut result = Mat4::identity();
    result[(0, 0)] = two * znear * x;
    result[(1, 1)] = two * znear * y;
    result[(0, 2)] = (right + left) * x;
    result[(1, 2)] = (top + bottom) * y;
    result[(3, 2)] = -one;
    result[(3, 3)] = T::zero();

    if zfar.is_finite() {
        let range_inv = one / (znear - zfar);
        result[(2, 2)] = (znear + zfar) * range_inv;
        result[(2, 3)] = two * znear * zfar * range_inv;
    } else {
        result[(2, 2)] = -one;
        result[(2, 3)] = -two * znear;
    }

    result
}

// endregion: Projection matrices

// region: Camera matrices

/// Calculates the `Mat4` model matrix for a camera at eye position looking at the center position with a given up direction.
///
/// # Examples
/// ```
/// # use munum::{transform, vec3, vec4, assert_float_eq};
/// let m = transform::target_to(vec3(0_f32, 2., 0.), vec3(0., 0.6, 0.), vec3(0., 0., -1.));
/// assert_float_eq!((m * vec4(0_f32, 2., 0., 1.)).as_ref(), &[0., 2., -2., 1.]);
/// assert_float_eq!((m * vec4(0_f32, 2., -1., 1.)).as_ref(), &[0., 1., -2., 1.]);
/// assert_float_eq!((m * vec4(1_f32, 2., 0., 1.)).as_ref(), &[1., 2., -2., 1.]);
/// assert_float_eq!((m * vec4(0_f32, 1., 0., 1.)).as_ref(), &[0., 2., -1., 1.]);
/// ```
#[cfg(any(feature = "std", feature = "libm"))]
pub fn target_to<T: Copy + Float + NumAssign>(
    eye: Vec3<T>,
    center: Vec3<T>,
    up: Vec3<T>,
) -> Mat4<T> {
    let mut v = eye - center; // front
    v.normalize();
    let mut n = up.cross(v); // right
    n.normalize();
    let mut u = v.cross(n); // up
    u.normalize();

    let mut result = Mat4::identity();
    for i in 0..3 {
        result[(i, 0)] = n[i];
        result[(i, 1)] = u[i];
        result[(i, 2)] = v[i];
        result[(i, 3)] = eye[i];
    }
    result
}

/// Calculate the 4x4 view matrix for a camera at eye position looking at the center position with a given up direction.
///
/// # Examples
/// ```
/// # use munum::{transform, vec3, vec4, assert_float_eq};
/// let m = transform::look_at(vec3(0_f32, 2., 0.), vec3(0., 0.6, 0.), vec3(0., 0., -1.));
/// assert_float_eq!((m * vec4(0_f32, 2., 0., 1.)).as_ref(), &[0., 0., 0., 1.]);
/// assert_float_eq!((m * vec4(0_f32, 2., -1., 1.)).as_ref(), &[0., 1., 0., 1.]);
/// assert_float_eq!((m * vec4(1_f32, 2., 0., 1.)).as_ref(), &[1., 0., 0., 1.]);
/// assert_float_eq!((m * vec4(0_f32, 1., 0., 1.)).as_ref(), &[0., 0., -1., 1.]);
/// ```
#[cfg(any(feature = "std", feature = "libm"))]
pub fn look_at<T: Copy + Float + NumAssign>(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Mat4<T> {
    let mut v = center - eye; // front
    v.normalize();
    let mut n = v.cross(up); // right
    n.normalize();
    let mut u = n.cross(v); // up
    u.normalize();

    let mut result = Mat4::identity();
    for i in 0..3 {
        result[(0, i)] = n[i];
        result[(1, i)] = u[i];
        result[(2, i)] = -v[i];
    }
    result[(0, 3)] = -n.dot(eye);
    result[(1, 3)] = -u.dot(eye);
    result[(2, 3)] = v.dot(eye);
    result
}

// endregion: Camera matrices
