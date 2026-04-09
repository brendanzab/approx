use super::RelativeEq;


#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
macro_rules! impl_relative_eq {
    () => {
        impl RelativeEq for () {
            fn default_max_relative() -> Self::Epsilon {
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