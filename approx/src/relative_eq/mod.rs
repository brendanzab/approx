pub mod trait_def;
pub mod core_impls;
pub mod tuples;
pub mod externals;
pub mod collections;
pub mod primitives;

pub use trait_def::RelativeEq;
#[cfg(feature = "vec_impl")]
use alloc::vec::Vec;
#[cfg(feature = "indexmap_impl")]
use core::hash::{BuildHasher, Hash};
#[cfg(feature = "indexmap_impl")]
use indexmap::IndexMap;
#[cfg(feature = "num_complex")]
use num_complex::Complex;

#[cfg(feature = "ordered_float")]
use num_traits::Float;
#[cfg(feature = "ordered_float")]
use externals::{NotNan, OrderedFloat};

