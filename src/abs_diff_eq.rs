#[cfg(feature="no_std")]
use core::{f32, f64};
#[cfg(not(feature="no_std"))]
use std::{f32, f64};
#[cfg(feature="no_std")]
#[cfg_attr(feature="no_std", allow(unused_imports))] // HACK: seems to be a bug in this lint!
use core::num::Float;
#[cfg(feature="use_complex")]
use num_complex::Complex;


/// Equality that is defined using the absolute difference of two numbers.
pub trait AbsDiffEq: PartialEq {
    /// Used for specifying relative comparisons.
    type Epsilon;

    /// The default tolerance to use when testing values that are close together.
    ///
    /// This is used when no `epsilon` value is supplied to the `abs_diff_eq!`, `relative_eq!`, or
    /// `ulps_eq!` macros.
    fn default_epsilon() -> Self::Epsilon;

    /// A test for equality that uses the absolute difference to compute the approximate
    /// equality of two numbers.
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool;

    /// The inverse of `ApproxEq::abs_diff_eq`.
    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////
// Base implementations
///////////////////////////////////////////////////////////////////////////////////////////////////


macro_rules! impl_unsigned_abs_diff_eq {
    ($T:ident, $default_epsilon:expr) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn default_epsilon() -> $T { $default_epsilon }

            #[inline]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                (if self > other { self - other } else { other - self }) <= epsilon
            }
        }
    }
}

impl_unsigned_abs_diff_eq!(u8, 0);
impl_unsigned_abs_diff_eq!(u16, 0);
impl_unsigned_abs_diff_eq!(u32, 0);
impl_unsigned_abs_diff_eq!(u64, 0);
impl_unsigned_abs_diff_eq!(usize, 0);

macro_rules! impl_signed_abs_diff_eq {
    ($T:ident, $default_epsilon:expr) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn default_epsilon() -> $T { $default_epsilon }

            #[inline]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                $T::abs(self - other) <= epsilon
            }
        }
    }
}

impl_signed_abs_diff_eq!(i8, 0);
impl_signed_abs_diff_eq!(i16, 0);
impl_signed_abs_diff_eq!(i32, 0);
impl_signed_abs_diff_eq!(i64, 0);
impl_signed_abs_diff_eq!(isize, 0);
impl_signed_abs_diff_eq!(f32, f32::EPSILON);
impl_signed_abs_diff_eq!(f64, f64::EPSILON);


///////////////////////////////////////////////////////////////////////////////////////////////////
// Derived implementations
///////////////////////////////////////////////////////////////////////////////////////////////////


impl<'a, T: AbsDiffEq> AbsDiffEq for &'a T {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&'a T, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(*self, *other, epsilon)
    }
}

impl<'a, T: AbsDiffEq> AbsDiffEq for &'a mut T {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&'a mut T, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(*self, *other, epsilon)
    }
}

#[cfg(feature="use_complex")]
impl<T: AbsDiffEq> AbsDiffEq for Complex<T>
    where T::Epsilon: Clone
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Complex<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.re, &other.re, epsilon.clone()) &&
        T::abs_diff_eq(&self.im, &other.im, epsilon.clone())
    }
}
