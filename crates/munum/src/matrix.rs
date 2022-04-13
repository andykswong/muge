use core::ops::{Index, IndexMut};
use core::slice;
use num::traits::NumAssign;

/// A column-major numeric matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Matrix<T: Copy + NumAssign, const R: usize, const C: usize>(pub(crate) [[T; R]; C]);

impl<T: Copy + NumAssign, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Creates a matrix from raw 2D array.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let m = Matrix::<f32, 2, 2>::new([[1., 2.], [3., 4.]]);
    /// assert_eq!(*m.as_ref(), [1., 2., 3., 4.]);
    /// ```
    #[inline]
    pub fn new(data: [[T; R]; C]) -> Self {
        Self(data)
    }

    /// Creates a matrix from slice.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let m = Matrix::<f32, 2, 2>::from_slice(&[1., 2., 3., 4.]);
    /// assert_eq!(*m.as_ref(), [1., 2., 3., 4.]);
    /// ```
    #[inline]
    pub fn from_slice(data: &[T]) -> Self {
        let mut result = Self::default();
        result.as_mut().clone_from_slice(data);
        result
    }

    /// Returns the number of columns in the matrix.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let m = Matrix::<f32, 3, 2>::default();
    /// assert_eq!(m.columns(), 2);
    /// ```
    #[inline]
    pub fn columns(&self) -> usize {
        C
    }

    /// Returns the number of rows in the matrix.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let m = Matrix::<f32, 3, 2>::default();
    /// assert_eq!(m.rows(), 3);
    /// ```
    #[inline]
    pub fn rows(&self) -> usize {
        R
    }
}

impl<T: Copy + NumAssign, const N: usize> Matrix<T, N, N> {
    /// Creates an identity matrix
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let m = Matrix::<f32, 3, 3>::identity();
    /// assert_eq!(*m.as_ref(), [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    /// ```
    pub fn identity() -> Self {
        let mut m = Self::default();
        for i in 0..N {
            m.0[i][i] = T::one();
        }
        m
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> Default for Matrix<T, R, C> {
    /// Creates a zero matrix.
    ///
    /// # Examples
    /// ```
    /// # use munum::Matrix;
    /// let m = Matrix::<f32, 3, 3>::default();
    /// assert_eq!(*m.as_ref(), [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    /// ```
    #[inline]
    fn default() -> Self {
        Self([[T::zero(); R]; C])
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> AsRef<[T]> for Matrix<T, R, C> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.0.as_ptr() as *const T, R * C) }
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> AsMut<[T]> for Matrix<T, R, C> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut T, R * C) }
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index / R][index % R]
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index / R][index % R]
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> Index<(usize, usize)>
    for Matrix<T, R, C>
{
    type Output = T;

    /// Indexing into the `Matrix` by (row, column).
    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[col][row]
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> IndexMut<(usize, usize)>
    for Matrix<T, R, C>
{
    /// Mutably indexing into the `Matrix` by (row, column).
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[col][row]
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> From<[[T; R]; C]> for Matrix<T, R, C> {
    #[inline]
    fn from(data: [[T; R]; C]) -> Self {
        Self::new(data)
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> From<&[T]> for Matrix<T, R, C> {
    #[inline]
    fn from(slice: &[T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T: Copy + NumAssign, const R: usize, const C: usize> From<Matrix<T, R, C>> for [[T; R]; C] {
    #[inline]
    fn from(m: Matrix<T, R, C>) -> Self {
        m.0
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use core::fmt;
    use core::marker::PhantomData;

    use super::Matrix;
    use num::traits::NumAssign;
    use serde::{
        de::{SeqAccess, Visitor},
        ser::SerializeSeq,
        Deserialize, Deserializer, Serialize, Serializer,
    };

    impl<T: Copy + NumAssign, const R: usize, const C: usize> Serialize for Matrix<T, R, C>
    where
        T: Serialize,
    {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut seq = serializer.serialize_seq(Some(R * C))?;
            for c in 0..C {
                for r in 0..R {
                    seq.serialize_element(&self.0[c][r])?;
                }
            }
            seq.end()
        }
    }

    struct MatrixArrayDeserializer<T: Copy + NumAssign, const R: usize, const C: usize>(
        PhantomData<[[T; R]; C]>,
    );

    impl<'de, T, const R: usize, const C: usize> Visitor<'de> for MatrixArrayDeserializer<T, R, C>
    where
        T: Deserialize<'de> + Copy + NumAssign,
    {
        type Value = [[T; R]; C];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("T sequence.")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut arr = [[T::zero(); R]; C];
            for c in 0..C {
                for r in 0..R {
                    if let Some(value) = seq.next_element()? {
                        arr[c][r] = value;
                    }
                }
            }
            Ok(arr)
        }
    }

    impl<'de, T: Copy + NumAssign, const R: usize, const C: usize> Deserialize<'de> for Matrix<T, R, C>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let arr = deserializer.deserialize_seq(MatrixArrayDeserializer(PhantomData))?;
            Ok(Matrix::new(arr))
        }
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use alloc::vec;
    use serde_json::{json, Value};

    use super::Matrix;

    #[test]
    fn test_serialize() {
        let mat = Matrix::<f32, 3, 3>::new([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        let expected_json: Value = json!([1., 2., 3., 4., 5., 6., 7., 8., 9.]);

        let json: Value = serde_json::to_value(mat).unwrap();
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_deserialize() {
        let json: Value = json!([1., 2., 3., 4., 5., 6., 7., 8., 9.]);

        let mat: Matrix<f32, 3, 3> = serde_json::from_value(json).unwrap();

        assert_eq!(*mat.as_ref(), [1., 2., 3., 4., 5., 6., 7., 8., 9.]);
    }
}
