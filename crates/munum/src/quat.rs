use core::f32::consts::PI;
use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use num::traits::{NumAssign, NumCast, One, Zero};

#[cfg(any(feature = "std", feature = "libm"))]
use num::traits::Float;

use crate::{float_eq, scalar, FloatEq, Mat3, Vec3, Vec4};

/// A quaternion in (x, y, z, w) order, where q = w + xi + yj + zk.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Quaternion<T: Copy + NumAssign = f32>(pub(crate) Vec4<T>);

/// Creates a quaternion from (x, y, z, w).
///
/// # Examples
/// ```
/// # use munum::quat;
/// let q = quat(1., 2., 3., 4.);
/// assert_eq!(*q.as_ref(), [1., 2., 3., 4.]);
/// ```
pub fn quat<T: Copy + NumAssign>(x: T, y: T, z: T, w: T) -> Quaternion<T> {
    Quaternion::from_slice(&[x, y, z, w])
}

impl<T: Copy + NumAssign> Quaternion<T> {
    /// Creates a new quaternion.
    ///
    /// # Examples
    /// ```
    /// # use munum::Quaternion;
    /// let q = <Quaternion>::new(1., 2., 3., 4.);
    /// assert_eq!(*q.as_ref(), [1., 2., 3., 4.]);
    /// ```
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self(Vec4::new([[x, y, z, w]]))
    }

    /// Creates a quaternion from raw array.
    ///
    /// # Examples
    /// ```
    /// # use munum::Quaternion;
    /// let q = <Quaternion>::from_array([1., 2., 3., 4.]);
    /// assert_eq!(*q.as_ref(), [1., 2., 3., 4.]);
    /// ```
    #[inline]
    pub fn from_array(arr: [T; 4]) -> Self {
        Self(Vec4::new([arr]))
    }

    /// Creates an identity quaternion.
    ///
    /// # Examples
    /// ```
    /// # use munum::Quaternion;
    /// let q = <Quaternion>::identity();
    /// assert_eq!(*q.as_ref(), [0.0, 0.0, 0.0, 1.0]);
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self(Vec4::new([[T::zero(), T::zero(), T::zero(), T::one()]]))
    }

    /// Creates a quaternion from slice.
    ///
    /// # Examples
    /// ```
    /// # use munum::Quaternion;
    /// let q = <Quaternion>::from_slice(&[1., 2., 3., 4.]);
    /// assert_eq!(*q.as_ref(), [1., 2., 3., 4.]);
    /// ```
    #[inline]
    pub fn from_slice(data: &[T]) -> Self {
        let mut result = Self::default();
        result.as_mut().clone_from_slice(data);
        result
    }
}

cfg_if::cfg_if! {
if #[cfg(any(feature = "std", feature = "libm"))] {
    impl<T: Copy + Float + NumAssign> Quaternion<T> {
        /// Creates a quaternion from a unit axis vector and rotation angle in couterclockwise direction.
        ///
        /// # Examples
        /// ```
        /// # use core::f32::consts::PI;
        /// # use munum::{Vec3, Quaternion, assert_float_eq};
        /// let q = <Quaternion>::from_axis_angle(<Vec3>::from_slice(&[3., 5., 7.]), PI/3.);
        /// let expected = <Quaternion>::from_slice(&[3. * (PI/6.).sin(), 5. * (PI/6.).sin(), 7. * (PI/6.).sin(), (PI/6.).cos()]);
        /// assert_float_eq!(q, expected, 0.00001);
        /// ```
        pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Self {
            let half_angle = angle / (T::one() + T::one());
            let sin_half_angle = half_angle.sin();
            Self::from_slice(&[axis[0] * sin_half_angle, axis[1] * sin_half_angle, axis[2] * sin_half_angle, half_angle.cos()])
        }

        /// Creates a quaternion from a rotation angle around x-axis in couterclockwise direction.
        ///
        /// # Examples
        /// ```
        /// # use core::f32::consts::PI;
        /// # use munum::{Quaternion, assert_float_eq};
        /// let q = <Quaternion>::from_angle_x(PI/3.);
        /// let expected = <Quaternion>::from_slice(&[(PI/6.).sin(), 0., 0., (PI/6.).cos()]);
        /// assert_float_eq!(q, expected, 0.00001);
        /// ```
        pub fn from_angle_x(angle: T) -> Self {
            let half_angle = angle / (T::one() + T::one());
            Self::from_slice(&[half_angle.sin(), T::zero(), T::zero(), half_angle.cos()])
        }

        /// Creates a quaternion from a rotation angle around x-axis in couterclockwise direction.
        ///
        /// # Examples
        /// ```
        /// # use core::f32::consts::PI;
        /// # use munum::{Quaternion, assert_float_eq};
        /// let q = <Quaternion>::from_angle_y(PI/3.);
        /// let expected = <Quaternion>::from_slice(&[0., (PI/6.).sin(), 0., (PI/6.).cos()]);
        /// assert_float_eq!(q, expected, 0.00001);
        /// ```
        pub fn from_angle_y(angle: T) -> Self {
            let half_angle = angle / (T::one() + T::one());
            Self::from_slice(&[T::zero(), half_angle.sin(), T::zero(), half_angle.cos()])
        }

        /// Creates a quaternion from a rotation angle around x-axis in couterclockwise direction.
        ///
        /// # Examples
        /// ```
        /// # use core::f32::consts::PI;
        /// # use munum::{Quaternion, assert_float_eq};
        /// let q = <Quaternion>::from_angle_z(PI/3.);
        /// let expected = <Quaternion>::from_slice(&[0., 0., (PI/6.).sin(), (PI/6.).cos()]);
        /// assert_float_eq!(q, expected, 0.00001);
        /// ```
        pub fn from_angle_z(angle: T) -> Self {
            let half_angle = angle / (T::one() + T::one());
            Self::from_slice(&[T::zero(), T::zero(), half_angle.sin(), half_angle.cos()])
        }
    }

    impl<T: Copy + Float + FloatEq<T> + NumAssign + NumCast> Quaternion<T> {
        /// Creates a quaternion that represents the shortest arc rotation between 2 unit vectors.
        ///
        /// # Examples
        /// ```
        /// # use core::f32::consts::PI;
        /// # use munum::{Quaternion, Vec3, assert_float_eq};
        /// let q = <Quaternion>::from_unit_vecs(<Vec3>::from_slice(&[0., 0., 1.]), <Vec3>::from_slice(&[1., 0., 0.]));
        /// let expected = <Quaternion>::from_slice(&[0., (PI/4.).sin(), 0., (PI/4.).cos()]);
        /// assert_float_eq!(q, expected, 0.00001);
        /// ```
        pub fn from_unit_vecs(from: Vec3<T>, to: Vec3<T>) -> Self {
            let epsilon = float_eq::epsilon();
            let dot = from.dot(to);

            if dot.float_eq(T::one(), epsilon) { // vectors are in same direction
                Self::identity()
            } else if dot.float_eq(-T::one(), epsilon) { // vectors are in parallel but opposite direction
                // use arbitrary perpendicular vector = (0, z, -y)
                let mut axis = Vec3::<T>::from_slice(&[T::zero(), from[2], -from[1]]);
                axis.normalize();
                Self::from_axis_angle(axis,  NumCast::from(PI).expect("incompatible type"))
            } else { // store perpendicular vector to the xyz of out.
                let cross = from.cross(to);
                let w = T::one() + dot;
                let mut result = Self::from_slice(&[cross[0], cross[1], cross[2], w]);
                result.0.normalize();
                result
            }
        }
    }
}
}

impl<T: Copy + NumAssign> Default for Quaternion<T> {
    #[inline]
    fn default() -> Self {
        Self(Vec4::default())
    }
}

impl<T: Copy + NumAssign> Zero for Quaternion<T> {
    #[inline]
    fn zero() -> Self {
        Self::default()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: Copy + NumAssign> One for Quaternion<T> {
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

impl<T: Copy + NumAssign> From<[T; 4]> for Quaternion<T> {
    #[inline]
    fn from(data: [T; 4]) -> Self {
        Self::from_array(data)
    }
}

impl<T: Copy + NumAssign> From<&[T]> for Quaternion<T> {
    #[inline]
    fn from(data: &[T]) -> Self {
        Self::from_slice(data)
    }
}

impl<T: Copy + NumAssign> From<Quaternion<T>> for [T; 4] {
    #[inline]
    fn from(q: Quaternion<T>) -> Self {
        q.0.into()
    }
}

impl<T: Copy + NumAssign> From<Quaternion<T>> for Mat3<T> {
    /// Converts a `Quaternion` into a `Mat3`
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat3, Quaternion, assert_float_eq};
    /// let ONE_OVER_SQRT3: f32 = 1. / 3_f32.sqrt();
    /// let m = <Mat3>::from(<Quaternion>::from_slice(&[ONE_OVER_SQRT3, ONE_OVER_SQRT3, ONE_OVER_SQRT3, 0.]));
    /// assert_float_eq!(m.as_ref(), &[-1./3., 2./3., 2./3., 2./3., -1./3., 2./3., 2./3., 2./3., -1./3.]);
    /// ```
    fn from(q: Quaternion<T>) -> Self {
        let one = T::one();
        let two = one + one;
        let xx = q[0] * q[0];
        let xy = q[0] * q[1];
        let xz = q[0] * q[2];
        let yy = q[1] * q[1];
        let yz = q[1] * q[2];
        let zz = q[2] * q[2];
        let wx = q[3] * q[0];
        let wy = q[3] * q[1];
        let wz = q[3] * q[2];

        Self::new([
            [one - two * (yy + zz), two * (xy + wz), two * (xz - wy)],
            [two * (xy - wz), one - two * (xx + zz), two * (yz + wx)],
            [two * (xz + wy), two * (yz - wx), one - two * (xx + yy)],
        ])
    }
}

impl<T: Copy + NumAssign> From<Vec3<T>> for Quaternion<T> {
    /// Creates a `Quaternion` from a `Vec3` using `w` = 0.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Vec3, Quaternion};
    /// let q = Quaternion::from(Vec3::<i32>::from_slice(&[2, 3, 4]));
    /// assert_eq!(*q.as_ref(), [2, 3, 4, 0]);
    /// ```
    #[inline]
    fn from(v: Vec3<T>) -> Self {
        Self::from_slice(&[v[0], v[1], v[2], T::zero()])
    }
}

impl<T: Copy + NumAssign> From<Quaternion<T>> for Vec3<T> {
    /// Creates a `Vec3` from a `Quaternion` by dropping the `w` component.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Vec3, Quaternion};
    /// let v = Vec3::from(Quaternion::<i32>::from_slice(&[2, 3, 4, 5]));
    /// assert_eq!(*v.as_ref(), [2, 3, 4]);
    /// ```
    #[inline]
    fn from(q: Quaternion<T>) -> Self {
        Self::new([[q[0], q[1], q[2]]])
    }
}

impl<T: Copy + NumAssign> AsRef<[T]> for Quaternion<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T: Copy + NumAssign> AsMut<[T]> for Quaternion<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T: Copy + NumAssign> Index<usize> for Quaternion<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Copy + NumAssign> IndexMut<usize> for Quaternion<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// region: Special Ops

impl<T: Copy + NumAssign> Quaternion<T> {
    /// Calculates the dot product of 2 `Quaternion`s.
    ///
    /// # Examples
    /// ```
    /// # use munum::Quaternion;
    /// let (v1, v2) = (Quaternion::<i32>::from_slice(&[29, 31, 37, 41]), Quaternion::<i32>::from_slice(&[43, 47, 53, 59]));
    /// assert_eq!(v1.dot(v2), 7084);
    /// ```
    #[inline]
    pub fn dot(&self, rhs: Self) -> T {
        self.0.dot(rhs.0)
    }

    /// Calculates the square length of a 2 `Quaternion`.
    ///
    /// # Examples
    /// ```
    /// # use munum::Quaternion;
    /// assert_eq!(Quaternion::<i32>::from_slice(&[2, 5, 14, 8]).sqr_len(), 289);
    /// ```
    #[inline]
    pub fn sqr_len(&self) -> T {
        self.0.sqr_len()
    }

    /// Transforms this `Quaternion` into its conjugate.
    ///
    /// # Examples
    /// ```
    /// # use munum::Quaternion;
    /// let mut q = Quaternion::<i32>::from_slice(&[2, 5, 14, 8]);
    /// q.conj();
    /// assert_eq!(*q.as_ref(), [-2, -5, -14, 8]);
    /// ```
    pub fn conj(&mut self) {
        let neg = scalar::neg();
        for i in 0..3 {
            self.0[(i, 0)] *= neg;
        }
    }

    /// Inverts this `Quaternion`.
    ///
    /// # Examples
    /// ```
    /// # use munum::Quaternion;
    /// let mut q = <Quaternion>::from_slice(&[2., 5., 14., 8.]);
    /// q.invert();
    /// assert_eq!(*q.as_ref(), [-2. / 289., -5. / 289., -14. / 289., 8. / 289.]);
    /// ```
    pub fn invert(&mut self) {
        self.conj();
        let len2 = self.sqr_len();
        if len2 != T::zero() {
            *self /= len2;
        }
    }

    /// Returns the result from rotating given `Vec3` by this `Quaternion`, using the formula v' = q * v * q^-1.
    /// See: <https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation#Using_quaternion_as_rotations>
    ///
    /// # Examples
    /// ```
    /// # use munum::{Quaternion, Vec3};
    /// let q = <Quaternion>::from_slice(&[0., 1./2_f32.sqrt(), 0., 1./2_f32.sqrt()]);
    /// let v = <Vec3>::from_slice(&[1. ,1., 1.]);
    /// assert_eq!(*q.rotate_vec3(v).as_ref(), [1., 1., -1.]);
    /// ```
    pub fn rotate_vec3(self, v: Vec3<T>) -> Vec3<T> {
        let mut inv_q = self;
        inv_q.invert();
        let mut result = self * Self::from(v);
        result *= inv_q;
        return result.into();
    }
}

cfg_if::cfg_if! {
if #[cfg(any(feature = "std", feature = "libm"))] {
    impl<T: Copy + Float + NumAssign> Quaternion<T> {
        /// Calculates the length of this `Quaternion`.
        ///
        /// # Examples
        /// ```
        /// # use munum::Quaternion;
        /// assert_eq!(<Quaternion>::from_slice(&[2., 5., 14., 8.]).len(), 17.);
        /// ```
        #[inline]
        pub fn len(&self) -> T {
            self.0.len()
        }

        /// Normizalizes this `Quaternion`.
        ///
        /// # Examples
        /// ```
        /// # use munum::{Quaternion, assert_float_eq};
        /// let mut q = <Quaternion>::from_slice(&[2., 5., 14., 8.]);
        /// q.normalize();
        /// assert_float_eq!(q.as_ref(), &[2./17., 5./17., 14./17., 8./17.]);
        /// ```
        #[inline]
        pub fn normalize(&mut self) {
            self.0.normalize()
        }

        /// Linear interpolates between 2 unit `Quaternion`s.
        ///
        /// # Examples
        /// ```
        /// # use core::f32::consts::PI;
        /// # use munum::Quaternion;
        /// let (q1, q2) = (<Quaternion>::from_slice(&[(PI/4.).sin(), 0., 0., (PI/4.).cos()]), <Quaternion>::from_slice(&[(PI/4.).sin(), 0., 0., -(PI/4.).cos()]));
        /// assert_eq!(*q1.lerp(q2, 0.5).as_ref(), [1., 0., 0., 0.]);
        /// ```
        pub fn lerp(&self, rhs: Self, t: T) -> Self {
            let cos = self.dot(rhs);  // calculate cosine from dot product
            let mag_rhs = if cos.is_sign_negative() { -T::one() } else { T::one() };
            let mut result = Self::default();
            for i in 0..4 {
                result.0[i] = scalar::lerp(self.0[i], rhs.0[i] * mag_rhs, t);
            }
            result.normalize();
            result
        }

        /// Shperical linear interpolates between 2 unit `Quaternion`s.
        ///
        /// # Examples
        /// ```
        /// # use core::f32::consts::PI;
        /// # use munum::{Quaternion, assert_float_eq};
        /// let (q1, q2) = (<Quaternion>::from_slice(&[(PI/6.).sin(), 0., 0., (PI/6.).cos()]), <Quaternion>::from_slice(&[-(PI/6.).cos(), 0., 0., -(PI/6.).sin()]));
        /// assert_float_eq!(q1.slerp(q2, 0.5).as_ref(), &[(PI/4.).sin(), 0., 0., (PI/4.).cos()]);
        /// ```
        pub fn slerp(&self, rhs: Self, t: T) -> Self {
            let epsilon = float_eq::epsilon();
            let one = T::one();
            let mut cos = self.dot(rhs);  // calculate cosine from dot product
            // use the shortest path
            let mag_rhs = if cos.is_sign_negative() {
                cos = cos.neg();
                -one
            } else {
                one
            };

            // initialize with linear interpolation
            let mut scale0 = one - t;
            let mut scale1 = t;

            // use spherical interpolation only if the quaternions are not very close
            if one - cos > epsilon {
                let theta = cos.acos();
                let sin_theta = theta.sin();
                scale0 = ((one - t) * theta).sin() / sin_theta;
                scale1 = (t * theta).sin() / sin_theta;
            }
            scale1 *= mag_rhs;

            let mut result = Self::default();
            for i in 0..4 {
                result.0[i] = self.0[i] * scale0 + rhs.0[i] * scale1;
            }
            result
        }
    }
}
}

// endregion: Special Ops

// region: Arithmetic Ops

impl<T: Copy + NumAssign> MulAssign<T> for Quaternion<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        MulAssign::mul_assign(&mut self.0, rhs);
    }
}

impl<T: Copy + NumAssign> DivAssign<T> for Quaternion<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.0.div_assign(rhs);
    }
}

impl<T: Copy + NumAssign> Mul<T> for Quaternion<T> {
    type Output = Quaternion<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self;
        result *= rhs;
        return result;
    }
}

impl<T: Copy + NumAssign> Div<T> for Quaternion<T> {
    type Output = Quaternion<T>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        let mut result = self;
        result /= rhs;
        return result;
    }
}

impl<T: Copy + NumAssign> AddAssign for Quaternion<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
    }
}

impl<T: Copy + NumAssign> SubAssign for Quaternion<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0);
    }
}

impl<T: Copy + NumAssign> MulAssign for Quaternion<T> {
    /// Calculate the Hamilton product of 2
    ///
    /// # Examples
    /// ```
    /// # use munum::{Quaternion};
    /// let mut q = Quaternion::<i32>::from_slice(&[2, 3, 5, -7]);
    /// q *= Quaternion::<i32>::from_slice(&[11, 13, -17, 19]);
    /// assert_eq!(*q.as_ref(), [-155, 55, 207, -109]);
    /// ```
    fn mul_assign(&mut self, rhs: Self) {
        let (ax, ay, az, aw) = (self.0[0], self.0[1], self.0[2], self.0[3]);
        let (bx, by, bz, bw) = (rhs.0[0], rhs.0[1], rhs.0[2], rhs.0[3]);
        self.0[0] = aw * bx + ax * bw + ay * bz - az * by;
        self.0[1] = aw * by + ay * bw + az * bx - ax * bz;
        self.0[2] = aw * bz + az * bw + ax * by - ay * bx;
        self.0[3] = aw * bw - ax * bx - ay * by - az * bz;
    }
}

impl<T: Copy + NumAssign> Add for Quaternion<T> {
    type Output = Quaternion<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        return result;
    }
}

impl<T: Copy + NumAssign> Sub for Quaternion<T> {
    type Output = Quaternion<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        return result;
    }
}

impl<T: Copy + NumAssign> Mul for Quaternion<T> {
    type Output = Quaternion<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result *= rhs;
        return result;
    }
}

// endregion: Arithmetic Ops

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use alloc::vec;
    use serde_json::{json, Value};

    use super::Quaternion;

    #[test]
    fn test_serialize() {
        let q = Quaternion::from_array([1., 2., 3., 4.]);
        let expected_json: Value = json!([1., 2., 3., 4.]);

        let json: Value = serde_json::to_value(q).unwrap();
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_deserialize() {
        let json: Value = json!([1., 2., 3., 4.]);

        let q: Quaternion = serde_json::from_value(json).unwrap();

        assert_eq!(*q.as_ref(), [1., 2., 3., 4.]);
    }
}
