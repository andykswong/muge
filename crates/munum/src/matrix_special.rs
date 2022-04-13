use num::traits::NumAssign;

use crate::{scalar, Matrix};

/// A column matrix aka vector
pub type Vector<T, const R: usize> = Matrix<T, R, 1>;

/// A 2D vector
pub type Vec2<T = f32> = Vector<T, 2>;

/// A 3D vector
pub type Vec3<T = f32> = Vector<T, 3>;

/// A 4D vector
pub type Vec4<T = f32> = Vector<T, 4>;

/// A 2x2 matrix
pub type Mat2<T = f32> = Matrix<T, 2, 2>;

/// A 3x3 matrix
pub type Mat3<T = f32> = Matrix<T, 3, 3>;

/// A 4x4 matrix
pub type Mat4<T = f32> = Matrix<T, 4, 4>;

// region: Constructors

impl<T: Copy + NumAssign, const R: usize> Vector<T, R> {
    /// Creates a vector from raw array.
    ///
    /// # Examples
    /// ```
    /// # use munum::Vector;
    /// let v = Vector::<f32, 4>::from_array([1., 2., 3., 4.]);
    /// assert_eq!(*v.as_ref(), [1., 2., 3., 4.]);
    /// ```
    pub fn from_array(arr: [T; R]) -> Self {
        Self([arr])
    }
}

/// Creates a `Vec2` from (x, y).
///
/// # Examples
/// ```
/// # use munum::vec2;
/// let v = vec2(1, 2);
/// assert_eq!(*v.as_ref(), [1, 2]);
/// ```
#[inline]
pub fn vec2<T: Copy + NumAssign>(x: T, y: T) -> Vec2<T> {
    Vec2::new([[x, y]])
}

/// Creates a `Vec3` from (x, y, z).
///
/// # Examples
/// ```
/// # use munum::vec3;
/// let v = vec3(1, 2, 3);
/// assert_eq!(*v.as_ref(), [1, 2, 3]);
/// ```
#[inline]
pub fn vec3<T: Copy + NumAssign>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3::new([[x, y, z]])
}

/// Creates a `Vec4` from (x, y, z, w).
///
/// # Examples
/// ```
/// # use munum::vec4;
/// let v = vec4(1, 2, 3, 4);
/// assert_eq!(*v.as_ref(), [1, 2, 3, 4]);
/// ```
#[inline]
pub fn vec4<T: Copy + NumAssign>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4::new([[x, y, z, w]])
}

/// Creates an identity `Mat2`.
///
/// # Examples
/// ```
/// # use munum::mat2;
/// assert_eq!(*mat2::<i32>().as_ref(), [1, 0, 0, 1]);
/// ```
#[inline]
pub fn mat2<T: Copy + NumAssign>() -> Mat2<T> {
    Mat2::identity()
}

/// Creates an identity `Mat3`.
///
/// # Examples
/// ```
/// # use munum::mat3;
/// assert_eq!(*mat3::<i32>().as_ref(), [1, 0, 0, 0, 1, 0, 0, 0, 1]);
/// ```
#[inline]
pub fn mat3<T: Copy + NumAssign>() -> Mat3<T> {
    Mat3::identity()
}

/// Creates an identity `Mat4`.
///
/// # Examples
/// ```
/// # use munum::mat4;
/// assert_eq!(*mat4::<i32>().as_ref(), [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
/// ```
#[inline]
pub fn mat4<T: Copy + NumAssign>() -> Mat4<T> {
    Mat4::identity()
}

// endregion: Constructors

// region: Conversions

impl<T: Copy + NumAssign, const R: usize> From<[T; R]> for Vector<T, R> {
    #[inline]
    fn from(data: [T; R]) -> Self {
        Self::from_array(data)
    }
}

impl<T: Copy + NumAssign, const R: usize> From<Vector<T, R>> for [T; R] {
    #[inline]
    fn from(v: Vector<T, R>) -> Self {
        v.0[0]
    }
}

impl<T: Copy + NumAssign> From<Mat3<T>> for Mat4<T> {
    /// Augments a `Mat3` into a `Mat4`
    /// The resulting `Mat4` contains the given `Mat3` on upper-left with the lower-right element = 1.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat3, Mat4};
    /// let m = Mat4::from(Mat3::<i32>::from_slice(&[2, 3, 4, 5, 6, 7, 8, 9, 10]));
    /// assert_eq!(*m.as_ref(), [2, 3, 4, 0, 5, 6, 7, 0, 8, 9, 10, 0, 0, 0, 0, 1]);
    /// ```
    fn from(m: Mat3<T>) -> Self {
        let mut m4 = Self::identity();
        for c in 0..3 {
            for r in 0..3 {
                m4[(r, c)] = m[(r, c)];
            }
        }
        m4
    }
}

impl<T: Copy + NumAssign> From<Mat4<T>> for Mat3<T> {
    /// Creates a `Mat3` from the upper-left 3x3 of a `Mat4`
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat3, Mat4};
    /// let m = Mat3::from(Mat4::<i32>::from_slice(&[2, 3, 4, 11, 5, 6, 7, 12, 8, 9, 10, 13, 14, 15, 0, 1]));
    /// assert_eq!(*m.as_ref(), [2, 3, 4, 5, 6, 7, 8, 9, 10]);
    /// ```
    fn from(m: Mat4<T>) -> Self {
        let mut m3 = Self::default();
        for c in 0..3 {
            for r in 0..3 {
                m3[(r, c)] = m[(r, c)];
            }
        }
        m3
    }
}

impl<T: Copy + NumAssign> Vec3<T> {
    /// Creates a new `Vec3` from a `Vec2` and z component.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Vec2, Vec3};
    /// let v = Vec2::<i32>::from_slice(&[2, 3]);
    /// assert_eq!(*Vec3::<i32>::from_vec2(v, 4).as_ref(), [2, 3, 4]);
    /// ```
    #[inline]
    pub fn from_vec2(v: Vec2<T>, z: T) -> Self {
        Vec3::new([[v.0[0][0], v.0[0][1], z]])
    }

    /// Returns a copy of the xy components as a `Vec2`.
    ///
    /// # Examples
    /// ```
    /// # use munum::Vec3;
    /// let v = Vec3::<i32>::from_slice(&[2, 3, 4]);
    /// assert_eq!(*v.xy().as_ref(), [2, 3]);
    /// ```
    #[inline]
    pub fn xy(&self) -> Vec2<T> {
        Vec2::new([[self.0[0][0], self.0[0][1]]])
    }
}

impl<T: Copy + NumAssign> Vec4<T> {
    /// Creates a new `Vec4` from a `Vec3` and w component.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Vec3, Vec4};
    /// let v = Vec3::<i32>::from_slice(&[2, 3, 4]);
    /// assert_eq!(*Vec4::<i32>::from_vec3(v, 1).as_ref(), [2, 3, 4, 1]);
    /// ```
    #[inline]
    pub fn from_vec3(v: Vec3<T>, w: T) -> Self {
        Vec4::new([[v.0[0][0], v.0[0][1], v.0[0][2], w]])
    }

    /// Returns a copy of the xy components as a `Vec2`.
    ///
    /// # Examples
    /// ```
    /// # use munum::Vec4;
    /// let v = Vec4::<i32>::from_slice(&[1, 2, 3, 4]);
    /// assert_eq!(*v.xy().as_ref(), [1, 2]);
    /// ```
    #[inline]
    pub fn xy(&self) -> Vec2<T> {
        Vec2::new([[self.0[0][0], self.0[0][1]]])
    }

    /// Returns a copy of the xyz components as a `Vec3`.
    ///
    /// # Examples
    /// ```
    /// # use munum::Vec4;
    /// let v = Vec4::<i32>::from_slice(&[1, 2, 3, 4]);
    /// assert_eq!(*v.xyz().as_ref(), [1, 2, 3]);
    /// ```
    #[inline]
    pub fn xyz(&self) -> Vec3<T> {
        Vec3::new([[self.0[0][0], self.0[0][1], self.0[0][2]]])
    }
}

// endregion: Conversions

// region: Special Ops

impl<T: Copy + NumAssign> Vec3<T> {
    /// Calculates the cross product of this vector with another vector.
    ///
    /// # Examples
    /// ```
    /// # use munum::Vec3;
    /// let (v1, v2) = (Vec3::<i32>::from_slice(&[2, 3, 4]), Vec3::<i32>::from_slice(&[5, 6, 7]));
    /// assert_eq!(*v1.cross(v2).as_ref(), [-3, 6, -3]);
    /// ```
    pub fn cross(&self, rhs: Self) -> Self {
        let x = self.0[0][1] * rhs.0[0][2] - rhs.0[0][1] * self.0[0][2];
        let y = self.0[0][2] * rhs.0[0][0] - rhs.0[0][2] * self.0[0][0];
        let z = self.0[0][0] * rhs.0[0][1] - rhs.0[0][0] * self.0[0][1];
        Self::new([[x, y, z]])
    }
}

impl<T: Copy + NumAssign> Mat2<T> {
    /// Calculates the determinant of this matrix.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat2};
    /// let m = Mat2::<i32>::from_slice(&[1, 2, 3, 4]);
    /// assert_eq!(m.det(), -2);
    /// ```
    pub fn det(&self) -> T {
        self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1]
    }

    /// Invert this matrix.
    /// If this matrix is not invertible, this method returns false and the matrix is unchanged.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat2};
    /// let mut m = <Mat2>::from_slice(&[1., 2., 3., 4.]);
    /// assert!(m.invert());
    /// assert_eq!(*m.as_ref(), [-2., 1., 1.5, -0.5]);
    /// ```
    pub fn invert(&mut self) -> bool {
        let det = self.det();
        if det == T::zero() {
            false
        } else {
            let neg = scalar::neg();
            let (m0, m1, m2, m3) = (
                self.0[1][1] / det,
                self.0[0][1] * neg / det,
                self.0[1][0] * neg / det,
                self.0[0][0] / det,
            );
            self.0[0][0] = m0;
            self.0[0][1] = m1;
            self.0[1][0] = m2;
            self.0[1][1] = m3;

            true
        }
    }
}

impl<T: Copy + NumAssign> Mat3<T> {
    /// Calculates the determinant of this matrix.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat3};
    /// let m = Mat3::<i32>::from_slice(&[1, 0, 5, 2, 1, 6, 3, 4, 0]);
    /// assert_eq!(m.det(), 1);
    /// ```
    pub fn det(&self) -> T {
        self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2])
            - self.0[1][0] * (self.0[0][1] * self.0[2][2] - self.0[2][1] * self.0[0][2])
            + self.0[2][0] * (self.0[0][1] * self.0[1][2] - self.0[1][1] * self.0[0][2])
    }

    /// Invert this matrix.
    /// If this matrix is not invertible, this method returns false and the matrix is unchanged.
    /// For formula, see: <https://en.wikipedia.org/wiki/Invertible_matrix#Inversion_of_3_%C3%97_3_matrices>
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat3};
    /// let mut m = <Mat3>::from_slice(&[1., 0., 5., 2., 1., 6., 3., 4., 0.]);
    /// assert!(m.invert());
    /// assert_eq!(*m.as_ref(), [-24., 20., -5., 18., -15., 4., 5., -4., 1.]);
    ///
    /// let mut m2 = <Mat3>::from_slice(&[1., 0., 1., 0., 1., 0., 0., 0., 0.]);
    /// assert!(!m2.invert());
    /// assert_eq!(*m2.as_ref(), [1., 0., 1., 0., 1., 0., 0., 0., 0.]);
    /// ```
    pub fn invert(&mut self) -> bool {
        let det = self.det();
        if det == T::zero() {
            false
        } else {
            let (m0, m1, m2, m3, m4, m5, m6, m7, m8) = (
                self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2],
                self.0[2][1] * self.0[0][2] - self.0[0][1] * self.0[2][2],
                self.0[0][1] * self.0[1][2] - self.0[1][1] * self.0[0][2],
                self.0[2][0] * self.0[1][2] - self.0[1][0] * self.0[2][2],
                self.0[0][0] * self.0[2][2] - self.0[2][0] * self.0[0][2],
                self.0[1][0] * self.0[0][2] - self.0[0][0] * self.0[1][2],
                self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1],
                self.0[2][0] * self.0[0][1] - self.0[0][0] * self.0[2][1],
                self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1],
            );
            self.0[0][0] = m0 / det;
            self.0[0][1] = m1 / det;
            self.0[0][2] = m2 / det;
            self.0[1][0] = m3 / det;
            self.0[1][1] = m4 / det;
            self.0[1][2] = m5 / det;
            self.0[2][0] = m6 / det;
            self.0[2][1] = m7 / det;
            self.0[2][2] = m8 / det;

            true
        }
    }

    /// Transforms this matrix into a normal matrix, which is the inverse transpose of itself.
    /// If this matrix is not invertible, this method returns false and the matrix is unchanged.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat3};
    /// let mut m = <Mat3>::from_slice(&[0., 0., 1., 1., 0., 0., 0., 1., 0.]);
    /// assert!(m.normal_matrix());
    /// assert_eq!(*m.as_ref(), [0., 0., 1., 1., 0., 0., 0., 1., 0.]);
    /// ```
    #[inline]
    pub fn normal_matrix(&mut self) -> bool {
        if !self.invert() {
            false
        } else {
            self.transpose();
            true
        }
    }
}

impl<T: Copy + NumAssign> Mat4<T> {
    /// Calculates the determinant of this matrix.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat4};
    /// let m = Mat4::<i32>::from_slice(&[1, 1, 1, -1, 1, 1, -1, 1, 1, -1, 1, 1, -1, 1, 1, 1]);
    /// assert_eq!(m.det(), -16);
    /// ```
    pub fn det(&self) -> T {
        let fa0 = self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1];
        let fa1 = self.0[0][0] * self.0[2][1] - self.0[2][0] * self.0[0][1];
        let fa2 = self.0[0][0] * self.0[3][1] - self.0[3][0] * self.0[0][1];
        let fa3 = self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1];
        let fa4 = self.0[1][0] * self.0[3][1] - self.0[3][0] * self.0[1][1];
        let fa5 = self.0[2][0] * self.0[3][1] - self.0[3][0] * self.0[2][1];
        let fb0 = self.0[0][2] * self.0[1][3] - self.0[1][2] * self.0[0][3];
        let fb1 = self.0[0][2] * self.0[2][3] - self.0[2][2] * self.0[0][3];
        let fb2 = self.0[0][2] * self.0[3][3] - self.0[3][2] * self.0[0][3];
        let fb3 = self.0[1][2] * self.0[2][3] - self.0[2][2] * self.0[1][3];
        let fb4 = self.0[1][2] * self.0[3][3] - self.0[3][2] * self.0[1][3];
        let fb5 = self.0[2][2] * self.0[3][3] - self.0[3][2] * self.0[2][3];

        fa0 * fb5 - fa1 * fb4 + fa2 * fb3 + fa3 * fb2 - fa4 * fb1 + fa5 * fb0
    }

    /// Invert this matrix.
    /// If this matrix is not invertible, this method returns false and the matrix is unchanged.
    ///
    /// # Examples
    /// ```
    /// # use munum::{Mat4};
    /// let mut m = <Mat4>::from_slice(&[1., 1., 1., -1., 1., 1., -1., 1., 1., -1., 1., 1., -1., 1., 1., 1.]);
    /// assert!(m.invert());
    /// assert_eq!(*m.as_ref(), [0.25, 0.25, 0.25, -0.25, 0.25, 0.25, -0.25, 0.25, 0.25, -0.25, 0.25, 0.25, -0.25, 0.25, 0.25, 0.25]);
    /// ```
    pub fn invert(&mut self) -> bool {
        let fa0 = self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1];
        let fa1 = self.0[0][0] * self.0[2][1] - self.0[2][0] * self.0[0][1];
        let fa2 = self.0[0][0] * self.0[3][1] - self.0[3][0] * self.0[0][1];
        let fa3 = self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1];
        let fa4 = self.0[1][0] * self.0[3][1] - self.0[3][0] * self.0[1][1];
        let fa5 = self.0[2][0] * self.0[3][1] - self.0[3][0] * self.0[2][1];
        let fb0 = self.0[0][2] * self.0[1][3] - self.0[1][2] * self.0[0][3];
        let fb1 = self.0[0][2] * self.0[2][3] - self.0[2][2] * self.0[0][3];
        let fb2 = self.0[0][2] * self.0[3][3] - self.0[3][2] * self.0[0][3];
        let fb3 = self.0[1][2] * self.0[2][3] - self.0[2][2] * self.0[1][3];
        let fb4 = self.0[1][2] * self.0[3][3] - self.0[3][2] * self.0[1][3];
        let fb5 = self.0[2][2] * self.0[3][3] - self.0[3][2] * self.0[2][3];

        let det = fa0 * fb5 - fa1 * fb4 + fa2 * fb3 + fa3 * fb2 - fa4 * fb1 + fa5 * fb0;
        if det == T::zero() {
            false
        } else {
            let zero = T::zero();
            let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15) = (
                self.0[1][1] * fb5 - self.0[2][1] * fb4 + self.0[3][1] * fb3,
                zero - self.0[0][1] * fb5 + self.0[2][1] * fb2 - self.0[3][1] * fb1,
                self.0[0][1] * fb4 - self.0[1][1] * fb2 + self.0[3][1] * fb0,
                zero - self.0[0][1] * fb3 + self.0[1][1] * fb1 - self.0[2][1] * fb0,
                zero - self.0[1][0] * fb5 + self.0[2][0] * fb4 - self.0[3][0] * fb3,
                self.0[0][0] * fb5 - self.0[2][0] * fb2 + self.0[3][0] * fb1,
                zero - self.0[0][0] * fb4 + self.0[1][0] * fb2 - self.0[3][0] * fb0,
                self.0[0][0] * fb3 - self.0[1][0] * fb1 + self.0[2][0] * fb0,
                self.0[1][3] * fa5 - self.0[2][3] * fa4 + self.0[3][3] * fa3,
                zero - self.0[0][3] * fa5 + self.0[2][3] * fa2 - self.0[3][3] * fa1,
                self.0[0][3] * fa4 - self.0[1][3] * fa2 + self.0[3][3] * fa0,
                zero - self.0[0][3] * fa3 + self.0[1][3] * fa1 - self.0[2][3] * fa0,
                zero - self.0[1][2] * fa5 + self.0[2][2] * fa4 - self.0[3][2] * fa3,
                self.0[0][2] * fa5 - self.0[2][2] * fa2 + self.0[3][2] * fa1,
                zero - self.0[0][2] * fa4 + self.0[1][2] * fa2 - self.0[3][2] * fa0,
                self.0[0][2] * fa3 - self.0[1][2] * fa1 + self.0[2][2] * fa0,
            );
            self.0[0][0] = m0 / det;
            self.0[0][1] = m1 / det;
            self.0[0][2] = m2 / det;
            self.0[0][3] = m3 / det;
            self.0[1][0] = m4 / det;
            self.0[1][1] = m5 / det;
            self.0[1][2] = m6 / det;
            self.0[1][3] = m7 / det;
            self.0[2][0] = m8 / det;
            self.0[2][1] = m9 / det;
            self.0[2][2] = m10 / det;
            self.0[2][3] = m11 / det;
            self.0[3][0] = m12 / det;
            self.0[3][1] = m13 / det;
            self.0[3][2] = m14 / det;
            self.0[3][3] = m15 / det;

            true
        }
    }
}

// endregion: Special Ops
