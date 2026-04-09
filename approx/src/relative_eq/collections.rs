#[cfg(feature = "array_impl")]
use crate::RelativeEq;

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



#[cfg(feature = "array_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "array_impl")))]
impl<A, B, const N: usize> RelativeEq<[B; N]> for [A; N]
where
    A: RelativeEq<B>,
    A::Epsilon: Clone,
{
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


