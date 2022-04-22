//! Helpers for cons tuple type.

use core::marker::PhantomData;

/// Constructs a [trait@Cons] based on the values or identifiers passed in.
///
/// # Examples
/// ```
/// # use muds::cons;
/// let (c1, (c2, (c3, ()))) = cons!(123f32, "hello", Some(45));
/// assert_eq!((c1, c2, c3), (123f32, "hello", Some(45)));
///
/// let cons!(c1, c2, c3) = cons!(123f32, "hello", Some(45));
/// assert_eq!(c3, Some(45));
///
/// let cons!(c1, c2, c3, ..rest) = cons!(1, 2, 3, 4, 5);
/// assert_eq!(rest, cons!(4, 5));
///
/// // rev[..] captures in reverse order.
/// let cons!(rev[c1, c2, c3, c4, c5]) = cons!(1, 2, 3, 4, 5);
/// assert_eq!([c1, c2, c3], [5, 4, 3]);
/// ```
#[macro_export]
macro_rules! cons {
    () => { () };

    // Base cases for rev, calls cons! normally
    (rev[$head:tt] $($reversed:tt)*) => {
        cons!($head, $($reversed)*)
    };
    (rev[$head:expr] $($reversed:tt)*) => {
        cons!($head, $($reversed)*)
    };
    (rev[$head:pat_param] $($reversed:tt)*) => {
        cons!($head, $($reversed)*)
    };

    // Recursively reverses the rev list
    (rev[$head:tt, $($rest:tt)*] $($reversed:tt)*) => {
        cons!(rev[$($rest)*] $head, $($reversed)*)
    };
    (rev[$head:expr, $($rest:tt)*] $($reversed:tt)*) => {
        cons!(rev[$($rest)*] $head, $($reversed)*)
    };
    (rev[$head:pat_param, $($rest:tt)*] $($reversed:tt)*) => {
        cons!(rev[$($rest)*] $head, $($reversed)*)
    };

    // Matches rest params
    (..$rest:tt) => { $rest };
    (..$rest:expr) => { $rest };

    // Base cases, returns single element cons
    ($head:tt) => { ($head, ()) };
    ($head:expr) => { ($head, ()) };
    ($head:pat_param) => { ($head, ()) };

    // Recursively builds the cons
    ($head:tt, $($tail:tt)*) => {
        ($head, cons!($($tail)*))
    };
    ($head:expr, $($tail:tt)*) => {
        ($head, cons!($($tail)*))
    };
    ($head:pat_param, $($tail:tt)*) => {
        ($head, cons!($($tail)*))
    };
}

/// Returns the concrete [trait@Cons] type signature for the provided types.
///
/// # Examples
/// ```
/// # use muds::{cons, Cons};
/// let c: Cons!(f32, &str, Option<i32>) = cons![123f32, "hello", Some(45)];
/// let c: Cons!(f32, ..Cons!(&str, Option<i32>)) = cons![123f32, "hello", Some(45)];
/// ```
#[macro_export]
macro_rules! Cons {
    () => { () };
    (..$Rest:ty) => { $Rest };
    ($A:ty) => { ($A, ()) };
    ($A:ty, $($Tail:tt)*) => {
        ($A, Cons!($($Tail)*))
    };
}

/// Trait for a [Cons](https://en.wikipedia.org/wiki/Cons).
pub trait Cons: Sized {
    const LEN: usize;

    /// Returns the length of cons.
    ///
    /// # Examples
    /// ```
    /// # use muds::{cons, collections::Cons};
    /// assert_eq!(cons!(1, 2, 3, 4, 5).len(), 5);
    /// ```
    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns if the cons is empty.
    ///
    /// # Examples
    /// ```
    /// # use muds::{cons, collections::Cons};
    /// assert!(().is_empty());
    /// assert!(!cons!(1, 2, 3, 4, 5).is_empty());
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }

    /// Gets an element by type from this cons.
    ///
    /// # Examples
    /// ```
    /// # use muds::{cons, collections::Cons};
    /// assert_eq!(*cons!(1f32, 1i32, 1u32).get::<i32, _>(), 1i32);
    /// ```
    #[inline]
    fn get<T, Index>(&self) -> &T
    where
        Self: ConsGetter<T, Index>,
    {
        ConsGetter::get(self)
    }

    /// Mutably gets an element by type from this cons.
    /// # Examples
    /// ```
    /// # use muds::{cons, collections::Cons};
    /// let mut c = cons!(1f32, 1i32, 1u32);
    /// *c.get_mut::<i32, _>() = 10;
    /// assert_eq!(c, cons!(1f32, 10i32, 1u32));
    /// ```
    #[inline]
    fn get_mut<T, Index>(&mut self) -> &mut T
    where
        Self: ConsGetter<T, Index>,
    {
        ConsGetter::get_mut(self)
    }

    /// Append to this cons.
    ///
    /// # Examples
    /// ```
    /// # use muds::{cons, collections::Cons};
    /// let cons!(c1, c2, c3, c4, c5) = cons!(1, 2).append(cons!(3, 4, 5));
    /// assert_eq!([c1, c2, c3, c4, c5], [1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    fn append<RHS: Cons>(self, rhs: RHS) -> <Self as Append<RHS>>::Output
    where
        Self: Append<RHS>,
    {
        Append::append(self, rhs)
    }

    /// Reverse this cons.
    //
    /// # Examples
    /// ```
    /// # use muds::{cons, collections::Cons};
    /// let cons!(c1, c2, c3, c4, c5) = cons!(1, 2, 3, 4, 5).rev();
    /// assert_eq!([c1, c2, c3, c4, c5], [5, 4, 3, 2, 1]);
    /// ```
    #[inline]
    fn rev(self) -> <Self as IntoRev>::Output
    where
        Self: IntoRev,
    {
        IntoRev::rev(self)
    }
}

impl Cons for () {
    const LEN: usize = 0;
}

impl<H, T: Cons> Cons for (H, T) {
    const LEN: usize = 1 + T::LEN;
}

/// Trait for appending to self.
pub trait Append<RHS> {
    /// Output type.
    type Output;

    /// Append to self.
    fn append(self, rhs: RHS) -> Self::Output;
}

impl<RHS> Append<RHS> for ()
where
    RHS: Cons,
{
    type Output = RHS;

    #[inline(always)]
    fn append(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, Tail, RHS> Append<RHS> for (H, Tail)
where
    Tail: Append<RHS>,
    RHS: Cons,
{
    type Output = (H, <Tail as Append<RHS>>::Output);

    #[inline(always)]
    fn append(self, rhs: RHS) -> Self::Output {
        (self.0, self.1.append(rhs))
    }
}

/// Trait for reversing self.
pub trait IntoRev {
    /// Output type.
    type Output;

    /// Revert self.
    fn rev(self) -> Self::Output;
}

impl IntoRev for () {
    type Output = ();

    #[inline(always)]
    fn rev(self) -> Self::Output {
        self
    }
}

impl<T, Tail> IntoRev for (T, Tail)
where
    Tail: IntoRev,
    <Tail as IntoRev>::Output: Append<(T, ())>,
{
    type Output = <<Tail as IntoRev>::Output as Append<(T, ())>>::Output;

    #[inline(always)]
    fn rev(self) -> Self::Output {
        self.1.rev().append((self.0, ()))
    }
}

/// Trait for getting a [trait@Cons] element by type.
pub trait ConsGetter<T, I> {
    /// Gets an element by type from cons.
    fn get(&self) -> &T;

    /// Mutably gets an element by type from cons.
    fn get_mut(&mut self) -> &mut T;
}

impl<T, Tail> ConsGetter<T, Here> for (T, Tail) {
    #[inline(always)]
    fn get(&self) -> &T {
        &self.0
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<Head, Tail, FromTail, TailIndex> ConsGetter<FromTail, There<TailIndex>> for (Head, Tail)
where
    Tail: ConsGetter<FromTail, TailIndex>,
{
    #[inline(always)]
    fn get(&self) -> &FromTail {
        self.1.get()
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut FromTail {
        self.1.get_mut()
    }
}

/// Used as an index into a [trait@Cons].
pub struct Here {
    _priv: (),
}

/// Used as an index into a [trait@Cons].
pub struct There<T> {
    _marker: PhantomData<T>,
}
