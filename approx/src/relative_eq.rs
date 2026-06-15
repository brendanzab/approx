use crate::AbsDiffEq;
#[cfg(feature = "vec_impl")]
use alloc::vec::Vec;
#[cfg(feature = "indexmap_impl")]
use core::hash::{BuildHasher, Hash};
use core::{cell, f32, f64};
#[cfg(feature = "indexmap_impl")]
use indexmap::IndexMap;
#[cfg(feature = "num_complex")]
use num_complex::Complex;

#[cfg(feature = "ordered_float")]
use num_traits::Float;
#[cfg(feature = "ordered_float")]
use ordered_float::{NotNan, OrderedFloat};

/// Equality comparisons between two numbers using both the absolute difference and
/// relative based comparisons.
///
/// For two number `a` and `b`, if `a` and `b` are epsilon equal under [AbsDiffEq] or if
/// `|a - b| <= max_relative * max(|a|, |b|)`, then the two numbers are considered to be
/// relative equal.
///
/// `relative_eq`, `relative_ne`, `assert_relative_eq`, and `assert_relative_ne` macros
/// are all wrappers of the `relative_eq` function in this trait.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate approx;
/// # fn main() {
/// assert_relative_eq!(1.0f32, 1.5f32, max_relative = 0.34);
/// assert_relative_ne!(1.0f32, 1.5f32, max_relative = 0.33);
/// # }
/// ```
pub trait RelativeEq<Rhs = Self>: AbsDiffEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The default epsilon for testing values that are close to zero
    ///
    /// This is used when no `epsilon` value is supplied to the
    /// [`relative_eq`](crate::relative_eq) macro.
    fn default_relative_epsilon() -> Self::Epsilon;

    /// The default relative tolerance for testing values that are far-apart.
    ///
    /// This is used when no `max_relative` value is supplied to the
    /// [`relative_eq`](crate::relative_eq) macro.
    fn default_max_relative() -> Self::Epsilon;

    /// A test for equality that uses a relative comparison if the values are far apart.
    fn relative_eq(&self, other: &Rhs, epsilon: Self::Epsilon, max_relative: Self::Epsilon)
        -> bool;

    /// The inverse of [`RelativeEq::relative_eq`].
    fn relative_ne(
        &self,
        other: &Rhs,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        !Self::relative_eq(self, other, epsilon, max_relative)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Base implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

// Implementation based on: [Comparing Floating Point Numbers, 2012 Edition]
// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
macro_rules! impl_relative_eq {
    ($T:ident) => {
        impl RelativeEq for $T {
            #[inline]
            fn default_relative_epsilon() -> $T {
                $T::MIN_POSITIVE
            }

            #[inline]
            fn default_max_relative() -> $T {
                $T::EPSILON
            }

            #[inline]
            #[allow(unused_imports)]
            fn relative_eq(&self, other: &$T, epsilon: $T, max_relative: $T) -> bool {
                use num_traits::float::FloatCore;
                // Handle same infinities
                if self == other {
                    return true;
                }

                // Handle remaining infinities
                if $T::is_infinite(*self) || $T::is_infinite(*other) {
                    return false;
                }

                let abs_diff = $T::abs(self - other);

                // For when the numbers are really close together
                if abs_diff <= epsilon {
                    return true;
                }

                let abs_self = $T::abs(*self);
                let abs_other = $T::abs(*other);

                let largest = if abs_other > abs_self {
                    abs_other
                } else {
                    abs_self
                };

                // Use a relative difference comparison
                abs_diff <= largest * max_relative
            }
        }
    };
}

impl_relative_eq!(f32);
impl_relative_eq!(f64);

///////////////////////////////////////////////////////////////////////////////////////////////////
// Derived implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T: RelativeEq> RelativeEq for Option<T> {
    #[inline]
    fn default_relative_epsilon() -> T::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Option<T>,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => T::relative_eq(a, b, epsilon, max_relative),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: RelativeEq, E: RelativeEq> RelativeEq for Result<T, E> {
    #[inline]
    fn default_relative_epsilon() -> (T::Epsilon, E::Epsilon) {
        (T::default_relative_epsilon(), E::default_relative_epsilon())
    }

    #[inline]
    fn default_max_relative() -> (T::Epsilon, E::Epsilon) {
        (T::default_max_relative(), E::default_max_relative())
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Result<T, E>,
        epsilon: (T::Epsilon, E::Epsilon),
        max_relative: (T::Epsilon, E::Epsilon),
    ) -> bool {
        match (self, other) {
            (Ok(a), Ok(b)) => T::relative_eq(a, b, epsilon.0, max_relative.0),
            (Err(a), Err(b)) => E::relative_eq(a, b, epsilon.1, max_relative.1),
            _ => false,
        }
    }
}

impl<'a, T: RelativeEq + ?Sized> RelativeEq for &'a T {
    #[inline]
    fn default_relative_epsilon() -> T::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &&'a T, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        T::relative_eq(*self, *other, epsilon, max_relative)
    }
}

impl<'a, T: RelativeEq + ?Sized> RelativeEq for &'a mut T {
    #[inline]
    fn default_relative_epsilon() -> T::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &&'a mut T,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        T::relative_eq(*self, *other, epsilon, max_relative)
    }
}

impl<T: RelativeEq + Copy> RelativeEq for cell::Cell<T> {
    #[inline]
    fn default_relative_epsilon() -> T::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &cell::Cell<T>,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        T::relative_eq(&self.get(), &other.get(), epsilon, max_relative)
    }
}

impl<T: RelativeEq + ?Sized> RelativeEq for cell::RefCell<T> {
    #[inline]
    fn default_relative_epsilon() -> T::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &cell::RefCell<T>,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        T::relative_eq(&self.borrow(), &other.borrow(), epsilon, max_relative)
    }
}

impl<A, B> RelativeEq<[B]> for [A]
where
    A: RelativeEq<B>,
    A::Epsilon: Clone,
{
    #[inline]
    fn default_relative_epsilon() -> A::Epsilon {
        A::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> A::Epsilon {
        A::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &[B], epsilon: A::Epsilon, max_relative: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other)
                .all(|(x, y)| A::relative_eq(x, y, epsilon.clone(), max_relative.clone()))
    }
}

#[cfg(feature = "array_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "array_impl")))]
impl<A, B, const N: usize> RelativeEq<[B; N]> for [A; N]
where
    A: RelativeEq<B>,
    A::Epsilon: Clone,
{
    #[inline]
    fn default_relative_epsilon() -> A::Epsilon {
        A::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> A::Epsilon {
        A::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &[B; N], epsilon: A::Epsilon, max_relative: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other)
                .all(|(x, y)| A::relative_eq(x, y, epsilon.clone(), max_relative.clone()))
    }
}

#[cfg(feature = "vec_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "vec_impl")))]
impl<A, B> RelativeEq<Vec<B>> for Vec<A>
where
    A: RelativeEq<B>,
    A::Epsilon: Clone,
{
    #[inline]
    fn default_relative_epsilon() -> A::Epsilon {
        A::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> A::Epsilon {
        A::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Vec<B>, epsilon: A::Epsilon, max_relative: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other)
                .all(|(x, y)| A::relative_eq(x, y, epsilon.clone(), max_relative.clone()))
    }
}

#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
macro_rules! impl_relative_eq {
    () => {
        impl RelativeEq for () {
            fn default_relative_epsilon() -> Self::Epsilon {
                ()
            }

            fn default_max_relative() -> Self::Epsilon {
                ()
            }

            fn relative_eq(
                &self,
                _other: &Self,
                _epsilon: Self::Epsilon,
                _max_relative: Self::Epsilon,
            ) -> bool {
                true
            }
        }
    };

    ($($idx:tt),+) => {
        paste::paste! {
            impl<$( [<T $idx>], )+> RelativeEq for ($( [<T $idx>], )+)
            where
                $( [<T $idx>]: RelativeEq, )+
            {
                fn default_relative_epsilon() -> Self::Epsilon {
                    ($( [<T $idx>]::default_relative_epsilon(), )+)
                }

                fn default_max_relative() -> Self::Epsilon {
                    ($( [<T $idx>]::default_max_relative(), )+)
                }

                fn relative_eq(
                    &self,
                    other: &Self,
                    epsilon: Self::Epsilon,
                    max_relative: Self::Epsilon,
                ) -> bool {
                    true $( && self.$idx.relative_eq(&other.$idx, epsilon.$idx, max_relative.$idx) )+
                }
            }
        }
    };
}

#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
mod relative_eq_tuple_impls {
    use super::*;

    impl_relative_eq!();
    impl_relative_eq!(0);
    impl_relative_eq!(0, 1);
    impl_relative_eq!(0, 1, 2);
    impl_relative_eq!(0, 1, 2, 3);
    impl_relative_eq!(0, 1, 2, 3, 4);
    impl_relative_eq!(0, 1, 2, 3, 4, 5);
    impl_relative_eq!(0, 1, 2, 3, 4, 5, 6);
    impl_relative_eq!(0, 1, 2, 3, 4, 5, 6, 7);
    impl_relative_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    impl_relative_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    impl_relative_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    impl_relative_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
}

#[cfg(feature = "num_complex")]
#[cfg_attr(docsrs, doc(cfg(feature = "num_complex")))]
impl<T: RelativeEq> RelativeEq for Complex<T>
where
    T::Epsilon: Clone,
{
    #[inline]
    fn default_relative_epsilon() -> T::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Complex<T>,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        T::relative_eq(&self.re, &other.re, epsilon.clone(), max_relative.clone())
            && T::relative_eq(&self.im, &other.im, epsilon, max_relative)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: RelativeEq + Copy> RelativeEq for NotNan<T> {
    #[inline]
    fn default_relative_epsilon() -> Self::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        T::relative_eq(
            &self.into_inner(),
            &other.into_inner(),
            epsilon,
            max_relative,
        )
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: RelativeEq + Float + ordered_float::FloatCore> RelativeEq<T> for NotNan<T> {
    #[inline]
    fn default_relative_epsilon() -> Self::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &T, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        T::relative_eq(&self.into_inner(), other, epsilon, max_relative)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: RelativeEq + Float + ordered_float::FloatCore> RelativeEq for OrderedFloat<T> {
    #[inline]
    fn default_relative_epsilon() -> Self::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        T::relative_eq(
            &self.into_inner(),
            &other.into_inner(),
            epsilon,
            max_relative,
        )
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: RelativeEq + Float + ordered_float::FloatCore> RelativeEq<T> for OrderedFloat<T> {
    #[inline]
    fn default_relative_epsilon() -> Self::Epsilon {
        T::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &T, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        T::relative_eq(&self.into_inner(), other, epsilon, max_relative)
    }
}

#[cfg(feature = "indexmap_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "indexmap_impl")))]
impl<K, V1, V2, S1, S2> RelativeEq<IndexMap<K, V2, S2>> for IndexMap<K, V1, S1>
where
    K: Hash + Eq,
    V1: RelativeEq<V2>,
    V1::Epsilon: Clone,
    S1: BuildHasher,
    S2: BuildHasher,
{
    #[inline]
    fn default_relative_epsilon() -> V1::Epsilon {
        V1::default_relative_epsilon()
    }

    #[inline]
    fn default_max_relative() -> V1::Epsilon {
        V1::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &IndexMap<K, V2, S2>,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, value)| {
                other.get(key).map_or(false, |v| {
                    V1::relative_eq(value, v, epsilon.clone(), max_relative.clone())
                })
            })
    }
}
