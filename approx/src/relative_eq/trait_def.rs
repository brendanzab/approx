use crate::AbsDiffEq;

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
