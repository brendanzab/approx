use super::trait_def::UlpsEq;
use crate::AbsDiffEq;
#[cfg(feature = "vec_impl")]
use alloc::vec::Vec;
use core::cell;
#[cfg(feature = "indexmap_impl")]
use core::hash::{BuildHasher, Hash};
#[cfg(feature = "indexmap_impl")]
use indexmap::IndexMap;
#[cfg(feature = "num_complex")]
use num_complex::Complex;
#[cfg(feature = "ordered_float")]
use num_traits::Float;
use num_traits::Signed;
#[cfg(feature = "ordered_float")]
use ordered_float::{NotNan, OrderedFloat};

// Implementation based on: [Comparing Floating Point Numbers, 2012 Edition]
// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
macro_rules! impl_ulps_eq {
    ($T:ident, $U:ident) => {
        impl UlpsEq for $T {
            #[inline]
            fn default_max_ulps() -> u32 {
                4
            }

            #[inline]
            fn ulps_eq(&self, other: &$T, epsilon: $T, max_ulps: u32) -> bool {
                // For when the numbers are really close together
                if $T::abs_diff_eq(self, other, epsilon) {
                    return true;
                }

                // Trivial negative sign check
                if self.signum() != other.signum() {
                    return false;
                }

                // ULPS difference comparison
                let int_self: $U = self.to_bits();
                let int_other: $U = other.to_bits();

                // To be replaced with `abs_sub`, if
                // https://github.com/rust-lang/rust/issues/62111 lands.
                if int_self <= int_other {
                    int_other - int_self <= max_ulps as $U
                } else {
                    int_self - int_other <= max_ulps as $U
                }
            }
        }
    };
}

impl_ulps_eq!(f32, u32);
impl_ulps_eq!(f64, u64);

impl<T: UlpsEq> UlpsEq for Option<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Option<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => T::ulps_eq(a, b, epsilon, max_ulps),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: UlpsEq, E: UlpsEq> UlpsEq for Result<T, E> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps().max(E::default_max_ulps())
    }

    #[inline]
    fn ulps_eq(
        &self,
        other: &Result<T, E>,
        epsilon: (T::Epsilon, E::Epsilon),
        max_ulps: u32,
    ) -> bool {
        match (self, other) {
            (Ok(a), Ok(b)) => T::ulps_eq(a, b, epsilon.0, max_ulps),
            (Err(a), Err(b)) => E::ulps_eq(a, b, epsilon.1, max_ulps),
            _ => false,
        }
    }
}

impl<'a, T: UlpsEq + ?Sized> UlpsEq for &'a T {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &&'a T, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(*self, *other, epsilon, max_ulps)
    }
}

impl<'a, T: UlpsEq + ?Sized> UlpsEq for &'a mut T {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &&'a mut T, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(*self, *other, epsilon, max_ulps)
    }
}

impl<T: UlpsEq + Copy> UlpsEq for cell::Cell<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &cell::Cell<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.get(), &other.get(), epsilon, max_ulps)
    }
}

impl<T: UlpsEq + ?Sized> UlpsEq for cell::RefCell<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &cell::RefCell<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.borrow(), &other.borrow(), epsilon, max_ulps)
    }
}

impl<A, B> UlpsEq<[B]> for [A]
where
    A: UlpsEq<B>,
    A::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        A::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &[B], epsilon: A::Epsilon, max_ulps: u32) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other)
                .all(|(x, y)| A::ulps_eq(x, y, epsilon.clone(), max_ulps))
    }
}

#[cfg(feature = "array_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "array_impl")))]
impl<A, B, const N: usize> UlpsEq<[B; N]> for [A; N]
where
    A: UlpsEq<B>,
    A::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        A::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &[B; N], epsilon: A::Epsilon, max_ulps: u32) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other)
                .all(|(x, y)| A::ulps_eq(x, y, epsilon.clone(), max_ulps))
    }
}

#[cfg(feature = "vec_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "vec_impl")))]
impl<A, B> UlpsEq<Vec<B>> for Vec<A>
where
    A: UlpsEq<B>,
    A::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        A::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Vec<B>, epsilon: A::Epsilon, max_ulps: u32) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other)
                .all(|(x, y)| A::ulps_eq(x, y, epsilon.clone(), max_ulps.clone()))
    }
}

#[cfg(feature = "num_complex")]
#[cfg_attr(docsrs, doc(cfg(feature = "num_complex")))]
impl<T: UlpsEq> UlpsEq for Complex<T>
where
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Complex<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.re, &other.re, epsilon.clone(), max_ulps)
            && T::ulps_eq(&self.im, &other.im, epsilon, max_ulps)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: UlpsEq + Copy> UlpsEq for NotNan<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &NotNan<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.into_inner(), &other.into_inner(), epsilon, max_ulps)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: UlpsEq + Float + ordered_float::FloatCore> UlpsEq<T> for NotNan<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &T, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.into_inner(), other, epsilon, max_ulps)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: UlpsEq + Float + ordered_float::FloatCore> UlpsEq for OrderedFloat<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &OrderedFloat<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.into_inner(), &other.into_inner(), epsilon, max_ulps)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: UlpsEq + Float + ordered_float::FloatCore> UlpsEq<T> for OrderedFloat<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &T, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.into_inner(), other, epsilon, max_ulps)
    }
}

#[cfg(feature = "indexmap_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "indexmap_impl")))]
impl<K, V1, V2, S1, S2> UlpsEq<IndexMap<K, V2, S2>> for IndexMap<K, V1, S1>
where
    K: Hash + Eq,
    V1: UlpsEq<V2>,
    V1::Epsilon: Clone,
    S1: BuildHasher,
    S2: BuildHasher,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        V1::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &IndexMap<K, V2, S2>, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, value)| {
                other.get(key).map_or(false, |v| {
                    V1::ulps_eq(value, v, epsilon.clone(), max_ulps.clone())
                })
            })
    }
}
