#[cfg(any(feature = "num_complex", feature = "ordered_float"))]
use super::RelativeEq;

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: RelativeEq + Copy> RelativeEq for NotNan<T> {
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
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &T, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        T::relative_eq(&self.into_inner(), other, epsilon, max_relative)
    }
}

#[cfg(feature = "num_complex")]
#[cfg_attr(docsrs, doc(cfg(feature = "num_complex")))]
impl<T: RelativeEq> RelativeEq for Complex<T>
where
    T::Epsilon: Clone,
{
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


