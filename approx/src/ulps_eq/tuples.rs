use super::UlpsEq;

#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
macro_rules! impl_ulps_eq {
    () => {
        impl UlpsEq for () {
            fn default_max_ulps() -> u32 {
                0
            }

            fn ulps_eq(
                &self,
                _other: &Self,
                _epsilon: Self::Epsilon,
                _max_ulps: u32,
            ) -> bool {
                true
            }
        }
    };

    ($($idx:tt),+) => {
        paste::paste! {
            impl<$( [<T $idx>], )+> UlpsEq for ($( [<T $idx>], )+)
            where
                $( [<T $idx>]: UlpsEq, )+
            {
                fn default_max_ulps() -> u32 {
                    0 $( .max([<T $idx>]::default_max_ulps()) )+
                }

                fn ulps_eq(
                    &self,
                    other: &Self,
                    epsilon: Self::Epsilon,
                    max_ulps: u32
                ) -> bool {
                    true $( && self.$idx.ulps_eq(&other.$idx, epsilon.$idx, max_ulps) )+
                }
            }
        }
    };
}

#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
mod ulps_eq_tuple_impls {
    use super::*;

    impl_ulps_eq!();
    impl_ulps_eq!(0);
    impl_ulps_eq!(0, 1);
    impl_ulps_eq!(0, 1, 2);
    impl_ulps_eq!(0, 1, 2, 3);
    impl_ulps_eq!(0, 1, 2, 3, 4);
    impl_ulps_eq!(0, 1, 2, 3, 4, 5);
    impl_ulps_eq!(0, 1, 2, 3, 4, 5, 6);
    impl_ulps_eq!(0, 1, 2, 3, 4, 5, 6, 7);
    impl_ulps_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    impl_ulps_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    impl_ulps_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    impl_ulps_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
}