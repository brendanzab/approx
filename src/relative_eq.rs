#[cfg(feature="no_std")]
use core::{f32, f64};
#[cfg(not(feature="no_std"))]
use std::{f32, f64};
#[cfg(feature="no_std")]
#[cfg_attr(feature="no_std", allow(unused_imports))] // HACK: seems to be a bug in this lint!
use core::num::Float;
#[cfg(feature="use_complex")]
use num_complex::Complex;

use AbsDiffEq;


/// Equality comparisons between two numbers using both the absolute difference and
/// relative based comparisons.
pub trait RelativeEq: AbsDiffEq {
    /// The default relative tolerance for testing values that are far-apart.
    ///
    /// This is used when no `max_relative` value is supplied to the `relative_eq` macro.
    fn default_max_relative() -> Self::Epsilon;

    /// A test for equality that uses a relative comparison if the values are far apart.
    fn relative_eq(&self,
                   other: &Self,
                   epsilon: Self::Epsilon,
                   max_relative: Self::Epsilon)
                   -> bool;

    /// The inverse of `ApproxEq::relative_eq`.
    fn relative_ne(&self,
                   other: &Self,
                   epsilon: Self::Epsilon,
                   max_relative: Self::Epsilon)
                   -> bool {
        !Self::relative_eq(self, other, epsilon, max_relative)
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////
// Base implementations
///////////////////////////////////////////////////////////////////////////////////////////////////


// Implementation based on: [Comparing Floating Point Numbers, 2012 Edition]
// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
macro_rules! impl_relative_eq {
    ($T:ident, $U:ident) => {
        impl RelativeEq for $T {
            #[inline]
            fn default_max_relative() -> $T { $T::EPSILON }

            #[inline]
            fn relative_eq(&self, other: &$T, epsilon: $T, max_relative: $T) -> bool {
                // Handle same infinities
                if self == other {
                    return true;
                }

                let abs_diff = $T::abs(self - other);

                // For when the numbers are really close together
                if abs_diff <= epsilon {
                    return true;
                }

                let abs_self = $T::abs(*self);
                let abs_other = $T::abs(*other);

                // Handle oppsite infinities
                if abs_self == abs_other && abs_diff == abs_self {
                    return false;
                }

                let largest = if abs_other > abs_self { abs_other } else { abs_self };

                // Use a relative difference comparison
                abs_diff <= largest * max_relative
            }
        }
    }
}

impl_relative_eq!(f32, i32);
impl_relative_eq!(f64, i64);


///////////////////////////////////////////////////////////////////////////////////////////////////
// Derived implementations
///////////////////////////////////////////////////////////////////////////////////////////////////


impl<'a, T: RelativeEq> RelativeEq for &'a T {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &&'a T, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        T::relative_eq(*self, *other, epsilon, max_relative)
    }
}

impl<'a, T: RelativeEq> RelativeEq for &'a mut T {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self,
                   other: &&'a mut T,
                   epsilon: T::Epsilon,
                   max_relative: T::Epsilon)
                   -> bool {
        T::relative_eq(*self, *other, epsilon, max_relative)
    }
}

#[cfg(feature="use_complex")]
impl<T: RelativeEq> RelativeEq for Complex<T>
    where T::Epsilon: Clone
{
    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self,
                   other: &Complex<T>,
                   epsilon: T::Epsilon,
                   max_relative: T::Epsilon)
                   -> bool {
        T::relative_eq(&self.re, &other.re, epsilon.clone(), max_relative.clone()) &&
        T::relative_eq(&self.im, &other.im, epsilon.clone(), max_relative.clone())
    }
}
