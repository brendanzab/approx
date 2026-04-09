use core::cell;

use crate::RelativeEq;

impl<T: RelativeEq> RelativeEq for Option<T> {
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
