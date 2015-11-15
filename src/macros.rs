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

#[macro_export]
macro_rules! relative_eq {
    ($lhs:expr, $rhs:expr, epsilon = $epsilon:expr, max_relative = $max_relative:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::relative_eq(lhs, rhs, $epsilon, $max_relative)
    }};
    ($lhs:expr, $rhs:expr, max_relative = $max_relative:expr, epsilon = $epsilon:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::relative_eq(lhs, rhs, $epsilon, $max_relative)
    }};
    ($lhs:expr, $rhs:expr, epsilon = $epsilon:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::relative_eq_with_epsilon(lhs, rhs, $epsilon)
    }};
    ($lhs:expr, $rhs:expr, max_relative = $max_relative:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::relative_eq_with_max(lhs, rhs, $max_relative)
    }};
    ($lhs:expr, $rhs:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::default_relative_eq(lhs, rhs)
    }};
}

#[macro_export]
macro_rules! relative_ne {
    ($lhs:expr, $rhs:expr, epsilon = $epsilon:expr, max_relative = $max_relative:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::relative_ne(lhs, rhs, $epsilon, $max_relative)
    }};
    ($lhs:expr, $rhs:expr, max_relative = $max_relative:expr, epsilon = $epsilon:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::relative_ne(lhs, rhs, $epsilon, $max_relative)
    }};
    ($lhs:expr, $rhs:expr, epsilon = $epsilon:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::relative_ne_with_epsilon(lhs, rhs, $epsilon)
    }};
    ($lhs:expr, $rhs:expr, max_relative = $max_relative:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::relative_ne_with_max(lhs, rhs, $max_relative)
    }};
    ($lhs:expr, $rhs:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::default_relative_ne(lhs, rhs)
    }};
}

#[macro_export]
macro_rules! assert_relative_eq {
    ($given:expr, $expected:expr) => {{
        let (given, expected) = (&($given), &($expected));

        if !relative_eq!(given, expected) {
            panic!(
"assert_relative_eq!({}, {})

    left = {:?}
    right = {:?}

",
                stringify!($given), stringify!($expected),
                given, expected,
            );
        }
    }};
    ($given:expr, $expected:expr, $($opt:ident = $opt_val:expr),+) => {{
        let (given, expected) = (&($given), &($expected));

        if !relative_eq!(given, expected, $($opt = $opt_val),+) {
            panic!(
"assert_relative_eq!({}, {}, {})

    left = {:?}
    right = {:?}

",
                stringify!($given), stringify!($expected),
                stringify!($($opt = $opt_val),+),
                given, expected,
            );
        }
    }};
    ($given:expr, $expected:expr,) => {
        assert_relative_eq!($given, $expected)
    };
    ($given:expr, $expected:expr, $($opt:ident = $opt_val:expr,)+) => {
        assert_relative_eq!($given, $expected, $($opt = $opt_val),+)
    };
}

#[macro_export]
macro_rules! assert_relative_ne {
    ($given:expr, $expected:expr) => {{
        let (given, expected) = (&($given), &($expected));

        if !relative_ne!(given, expected) {
            panic!(
"assert_relative_ne!({}, {})

    left = {:?}
    right = {:?}

",
                stringify!($given), stringify!($expected),
                given, expected,
            );
        }
    }};
    ($given:expr, $expected:expr, $($opt:ident = $opt_val:expr),+) => {{
        let (given, expected) = (&($given), &($expected));

        if !relative_ne!(given, expected, $($opt = $opt_val),+) {
            panic!(
"assert_relative_ne!({}, {}, {})

    left = {:?}
    right = {:?}

",
                stringify!($given), stringify!($expected),
                stringify!($($opt = $opt_val),+),
                given, expected,
            );
        }
    }};
    ($given:expr, $expected:expr,) => {
        assert_relative_ne!($given, $expected)
    };
}


#[macro_export]
macro_rules! ulps_eq {
    ($lhs:expr, $rhs:expr, epsilon = $epsilon:expr, max_ulps = $max_ulps:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::ulps_eq(lhs, rhs, $epsilon, $max_ulps)
    }};
    ($lhs:expr, $rhs:expr, max_ulps = $max_ulps:expr, epsilon = $epsilon:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::ulps_eq(lhs, rhs, $epsilon, $max_ulps)
    }};
    ($lhs:expr, $rhs:expr, epsilon = $epsilon:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::ulps_eq_with_epsilon(lhs, rhs, $epsilon)
    }};
    ($lhs:expr, $rhs:expr, max_ulps = $max_ulps:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::ulps_eq_with_max(lhs, rhs, $max_ulps)
    }};
    ($lhs:expr, $rhs:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::default_ulps_eq(lhs, rhs)
    }};
}

#[macro_export]
macro_rules! ulps_ne {
    ($lhs:expr, $rhs:expr, epsilon = $epsilon:expr, max_ulps = $max_ulps:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::ulps_ne(lhs, rhs, $epsilon, $max_ulps)
    }};
    ($lhs:expr, $rhs:expr, max_ulps = $max_ulps:expr, epsilon = $epsilon:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::ulps_ne(lhs, rhs, $epsilon, $max_ulps)
    }};
    ($lhs:expr, $rhs:expr, epsilon = $epsilon:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::ulps_ne_with_epsilon(lhs, rhs, $epsilon)
    }};
    ($lhs:expr, $rhs:expr, max_ulps = $max_ulps:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::ulps_ne_with_max(lhs, rhs, $max_ulps)
    }};
    ($lhs:expr, $rhs:expr) => {{
        let (lhs, rhs) = (&$lhs, &$rhs);
        $crate::macro_support::default_ulps_ne(lhs, rhs)
    }};
}

#[macro_export]
macro_rules! assert_ulps_eq {
    ($given:expr, $expected:expr) => {{
        let (given, expected) = (&($given), &($expected));

        if !ulps_eq!(given, expected) {
            panic!(
"assert_ulps_eq!({}, {})

    left = {:?}
    right = {:?}

",
                stringify!($given), stringify!($expected),
                given, expected,
            );
        }
    }};
    ($given:expr, $expected:expr, $($opt:ident = $opt_val:expr),+) => {{
        let (given, expected) = (&($given), &($expected));

        if !ulps_eq!(given, expected, $($opt = $opt_val),+) {
            panic!(
"assert_ulps_eq!({}, {}, {})

    left = {:?}
    right = {:?}

",
                stringify!($given), stringify!($expected),
                stringify!($($opt = $opt_val),+),
                given, expected,
            );
        }
    }};
    ($given:expr, $expected:expr,) => {
        assert_ulps_eq!($given, $expected)
    };
    ($given:expr, $expected:expr, $($opt:ident = $opt_val:expr,)+) => {
        assert_ulps_eq!($given, $expected, $($opt = $opt_val),+)
    };
}

#[macro_export]
macro_rules! assert_ulps_ne {
    ($given:expr, $expected:expr) => {{
        let (given, expected) = (&($given), &($expected));

        if !ulps_ne!(given, expected) {
            panic!(
"assert_ulps_ne!({}, {})

    left = {:?}
    right = {:?}

",
                stringify!($given), stringify!($expected),
                given, expected,
            );
        }
    }};
    ($given:expr, $expected:expr, $($opt:ident = $opt_val:expr),+) => {{
        let (given, expected) = (&($given), &($expected));

        if !ulps_ne!(given, expected, $($opt = $opt_val),+) {
            panic!(
"assert_ulps_ne!({}, {}, {})

    left = {:?}
    right = {:?}

",
                stringify!($given), stringify!($expected),
                stringify!($($opt = $opt_val),+),
                given, expected,
            );
        }
    }};
    ($given:expr, $expected:expr,) => {
        assert_ulps_ne!($given, $expected)
    };
}
