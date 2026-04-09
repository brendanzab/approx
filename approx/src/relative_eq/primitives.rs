use super::RelativeEq;


// Implementation based on: [Comparing Floating Point Numbers, 2012 Edition]
// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
macro_rules! impl_relative_eq {
    ($T:ident) => {
        impl RelativeEq for $T {
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