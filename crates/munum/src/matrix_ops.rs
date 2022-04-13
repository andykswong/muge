use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use num::traits::NumAssign;
use num::{One, Zero};

#[cfg(any(feature = "std", feature = "libm"))]
use num::traits::Float;

use crate::{scalar, Matrix};

impl<T: Copy + NumAssign, const R: usize, const C: usize> Zero for Matrix<T, R, C> {
    #[inline]
    fn zero() -> Self {
        Self::default()
    }

    fn is_zero(&self) -> bool {
        let zero_ele = T::zero();

        for c in 0..C {
            for r in 0..R {
                if self.0[c][r] != zero_ele {
                    return false;
                }
            }
        }
        true
    }
}

// region: Matrix Ops

impl<T: Copy + NumAssign, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Multiplies 2 matrices and stores the result into self.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let (m1, m2) = (Matrix::<i32, 3, 2>::from_slice(&[1, 2, 3, 4, 5, 6]), Matrix::<i32, 2, 1>::from_slice(&[7, 8]));
    /// let mut result = Matrix::<i32, 3, 1>::default();
    /// result.mul_assign(m1, m2);
    /// assert_eq!(*result.as_ref(), [39, 54, 69]);
    /// ```
    pub fn mul_assign<const N: usize>(&mut self, lhs: Matrix<T, R, N>, rhs: Matrix<T, N, C>) {
        for c in 0..C {
            for r in 0..R {
                let mut sum = T::zero();
                for n in 0..N {
                    sum += lhs[(r, n)] * rhs[(n, c)];
                }
                self[(r, c)] = sum;
            }
        }
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> AddAssign for Matrix<T, R, C> {
    fn add_assign(&mut self, rhs: Matrix<T, R, C>) {
        for c in 0..C {
            for r in 0..R {
                self.0[c][r] += rhs.0[c][r];
            }
        }
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> SubAssign for Matrix<T, R, C> {
    fn sub_assign(&mut self, rhs: Matrix<T, R, C>) {
        for c in 0..C {
            for r in 0..R {
                self.0[c][r] -= rhs.0[c][r];
            }
        }
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> Add for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    #[inline]
    fn add(self, rhs: Matrix<T, R, C>) -> Self::Output {
        let mut result = self;
        result += rhs;
        return result;
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> Sub for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    #[inline]
    fn sub(self, rhs: Matrix<T, R, C>) -> Self::Output {
        let mut result = self;
        result -= rhs;
        return result;
    }
}

impl<T: Copy + NumAssign, const R: usize, const N: usize, const C: usize> Mul<Matrix<T, N, C>>
    for Matrix<T, R, N>
{
    type Output = Matrix<T, R, C>;

    #[inline]
    fn mul(self, rhs: Matrix<T, N, C>) -> Self::Output {
        let mut result = Self::Output::default();
        result.mul_assign(self, rhs);
        result
    }
}

// endregion: Matrix Ops

// region: Scalar Ops

impl<T: Copy + NumAssign, const R: usize, const C: usize> MulAssign<T> for Matrix<T, R, C> {
    fn mul_assign(&mut self, rhs: T) {
        for c in 0..C {
            for r in 0..R {
                self.0[c][r] *= rhs;
            }
        }
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> DivAssign<T> for Matrix<T, R, C> {
    fn div_assign(&mut self, rhs: T) {
        for c in 0..C {
            for r in 0..R {
                self.0[c][r] /= rhs;
            }
        }
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> RemAssign<T> for Matrix<T, R, C> {
    fn rem_assign(&mut self, rhs: T) {
        for c in 0..C {
            for r in 0..R {
                self.0[c][r] %= rhs;
            }
        }
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self;
        result *= rhs;
        return result;
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> Div<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        let mut result = self;
        result /= rhs;
        return result;
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> Rem<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    #[inline]
    fn rem(self, rhs: T) -> Self::Output {
        let mut result = self;
        result %= rhs;
        return result;
    }
}

// endregion: Scalar Ops

// region: Square Matrix Ops

impl<T: Copy + NumAssign, const N: usize> One for Matrix<T, N, N> {
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

impl<T: Copy + NumAssign, const N: usize> MulAssign for Matrix<T, N, N> {
    #[inline]
    fn mul_assign(&mut self, rhs: Matrix<T, N, N>) {
        self.mul_assign(self.clone(), rhs);
    }
}

impl<T: Copy + NumAssign, const N: usize> Matrix<T, N, N> {
    /// Transpose the `Matrix`.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let mut m = Matrix::<i32, 3, 3>::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// m.transpose();
    /// assert_eq!(*m.as_ref(), [1, 4, 7, 2, 5, 8, 3, 6, 9]);
    /// ```
    pub fn transpose(&mut self) {
        for c in 0..N {
            for r in (c + 1)..N {
                // swap m[(r, c)] with m[(c, r)]
                let mrc = self.0[c][r];
                self.0[c][r] = self.0[r][c];
                self.0[r][c] = mrc;
            }
        }
    }

    /// Returns the `Matrix` transposed.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let mut m = Matrix::<i32, 3, 3>::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// assert_eq!(*m.transposed().as_ref(), [1, 4, 7, 2, 5, 8, 3, 6, 9]);
    /// ```
    #[inline]
    pub fn transposed(&self) -> Self {
        let mut m = *self;
        m.transpose();
        m
    }
}

// endregion: Square Matrix Ops

// region: Vector Ops

impl<T: Copy + NumAssign, const N: usize> Matrix<T, N, 1> {
    /// Calculates the dot product of 2 column matrices aka vectors.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let (v1, v2) = (Matrix::<i32, 3, 1>::from_slice(&[1, 2, 3]), Matrix::<i32, 3, 1>::from_slice(&[4, 5, 6]));
    /// assert_eq!(v1.dot(v2), 32);
    /// ```
    pub fn dot(&self, rhs: Self) -> T {
        let mut result = T::zero();
        for i in 0..N {
            result += self.0[0][i] * rhs.0[0][i];
        }
        result
    }

    /// Calculates the square length of a column matrix aka vector.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// assert_eq!(Matrix::<i32, 3, 1>::from_slice(&[3, 4, 12]).sqr_len(), 169);
    /// ```
    #[inline]
    pub fn sqr_len(&self) -> T {
        self.dot(*self)
    }

    /// Linear interpolates between 2 column matrices aka vectors.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let (v1, v2) = (Matrix::<f32, 3, 1>::from_slice(&[1., 2., 3.]), Matrix::<f32, 3, 1>::from_slice(&[5., 6., 7.]));
    /// assert_eq!(*v1.lerp(v2, 0.5).as_ref(), [3., 4., 5.]);
    /// ```
    pub fn lerp(&self, rhs: Self, t: T) -> Self {
        let mut result = Self::default();
        for i in 0..N {
            result.0[0][i] = scalar::lerp(self.0[0][i], rhs.0[0][i], t);
        }
        result
    }
}

cfg_if::cfg_if! {
if #[cfg(any(feature = "std", feature = "libm"))] {
    impl<T: Copy + Float + NumAssign, const N: usize> Matrix<T, N, 1> {
        /// Calculates the length of a column matrix aka vector.
        ///
        /// # Examples
        /// ```
        /// # use munum::Matrix;
        /// assert_eq!(Matrix::<f32, 3, 1>::from_slice(&[3., 4., 12.]).len(), 13.);
        /// ```
        #[inline]
        pub fn len(&self) -> T {
            self.dot(*self).sqrt()
        }

        /// Normizalizes this column matrix aka vector.
        ///
        /// # Examples
        /// ```
        /// # use munum::Matrix;
        /// let mut v = Matrix::<f32, 2, 1>::from_slice(&[3., 4.]);
        /// v.normalize();
        /// assert_eq!(*v.as_ref(), [0.6, 0.8]);
        /// ```
        pub fn normalize(&mut self) {
            let len = self.len();
            if len != T::zero() {
                *self /= len;
            }
        }

        /// Returns a normalized version of this vector.
        ///
        /// # Examples
        /// ```
        /// # use munum::Matrix;
        /// let mut v = Matrix::<f32, 2, 1>::from_slice(&[3., 4.]);
        /// assert_eq!(*v.normalized().as_ref(), [0.6, 0.8]);
        /// ```
        #[inline]
        pub fn normalized(&self) -> Self {
            let mut v = *self;
            v.normalize();
            v
        }
    }
}
}

// endregion: Vector Ops
