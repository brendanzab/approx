// Copyright 2015 Brendan Zabarauskas
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Functions that support the various forms of the `relative_eq!` and `ulps_eq!`
//! macros. These are not intended to be called directly.

use ApproxEq;

#[inline]
pub fn relative_eq<T: ApproxEq>(a: &T, b: &T, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
    T::relative_eq(a, b, epsilon, max_relative)
}

#[inline]
pub fn relative_eq_with_epsilon<T: ApproxEq>(a: &T, b: &T, epsilon: T::Epsilon) -> bool {
    T::relative_eq(a, b, epsilon, T::default_max_relative())
}

#[inline]
pub fn relative_eq_with_max<T: ApproxEq>(a: &T, b: &T, max_relative: T::Epsilon) -> bool {
    T::relative_eq(a, b, T::default_epsilon(), max_relative)
}

#[inline]
pub fn default_relative_eq<T: ApproxEq>(a: &T, b: &T) -> bool {
    T::relative_eq(a, b, T::default_epsilon(), T::default_max_relative())
}


#[inline]
pub fn relative_ne<T: ApproxEq>(a: &T, b: &T, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
    T::relative_ne(a, b, epsilon, max_relative)
}

#[inline]
pub fn relative_ne_with_epsilon<T: ApproxEq>(a: &T, b: &T, epsilon: T::Epsilon) -> bool {
    T::relative_ne(a, b, epsilon, T::default_max_relative())
}

#[inline]
pub fn relative_ne_with_max<T: ApproxEq>(a: &T, b: &T, max_relative: T::Epsilon) -> bool {
    T::relative_ne(a, b, T::default_epsilon(), max_relative)
}

#[inline]
pub fn default_relative_ne<T: ApproxEq>(a: &T, b: &T) -> bool {
    T::relative_ne(a, b, T::default_epsilon(), T::default_max_relative())
}


#[inline]
pub fn ulps_eq<T: ApproxEq>(a: &T, b: &T, epsilon: T::Epsilon, max_ulps: u32) -> bool {
    T::ulps_eq(a, b, epsilon, max_ulps)
}

#[inline]
pub fn ulps_eq_with_epsilon<T: ApproxEq>(a: &T, b: &T, epsilon: T::Epsilon) -> bool {
    T::ulps_eq(a, b, epsilon, T::default_max_ulps())
}

#[inline]
pub fn ulps_eq_with_max<T: ApproxEq>(a: &T, b: &T, max_ulps: u32) -> bool {
    T::ulps_eq(a, b, T::default_epsilon(), max_ulps)
}

#[inline]
pub fn default_ulps_eq<T: ApproxEq>(a: &T, b: &T) -> bool {
    T::ulps_eq(a, b, T::default_epsilon(), T::default_max_ulps())
}


#[inline]
pub fn ulps_ne<T: ApproxEq>(a: &T, b: &T, epsilon: T::Epsilon, max_ulps: u32) -> bool {
    T::ulps_ne(a, b, epsilon, max_ulps)
}

#[inline]
pub fn ulps_ne_with_epsilon<T: ApproxEq>(a: &T, b: &T, epsilon: T::Epsilon) -> bool {
    T::ulps_ne(a, b, epsilon, T::default_max_ulps())
}

#[inline]
pub fn ulps_ne_with_max<T: ApproxEq>(a: &T, b: &T, max_ulps: u32) -> bool {
    T::ulps_ne(a, b, T::default_epsilon(), max_ulps)
}

#[inline]
pub fn default_ulps_ne<T: ApproxEq>(a: &T, b: &T) -> bool {
    T::ulps_ne(a, b, T::default_epsilon(), T::default_max_ulps())
}
