// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

/// Construct a `cid::Ipld` roughly matching JSON format. This code is a modified version
/// of `serde_json::json` macro, with extensions for being able to indicate links and bytes.
/// These two matterns can be matched by wrapping `Cid` link in `Link(..)` or the `Vec<u8>` in
/// `Bytes()`.
///
/// ```
/// # use forest_ipld::ipld;
/// # use cid::Cid;
/// #
/// let value = ipld!({
///     "code": 200,
///     "success": true,
///     "link": Link("QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n".parse().unwrap()),
///     "bytes": Bytes(vec![0x1, 0xfa, 0x8b]),
///     "payload": {
///         "features": [
///             "serde",
///             "ipld"
///         ]
///     }
/// });
/// ```
///
/// Variables or expressions can be interpolated into the JSON literal. Any type
/// interpolated into an array element or object value must implement Serde's
/// `Serialize` trait, while any type interpolated into a object key must
/// implement `Into<String>`. If the `Serialize` implementation of the
/// interpolated type decides to fail, or if the interpolated type contains a
/// map with non-string keys, the `ipld!` macro will panic.
///
/// ```
/// # use forest_ipld::ipld;
/// #
/// let code = 200;
/// let features = vec!["serde", "ipld"];
///
/// let value = ipld!({
///     "code": code,
///     "success": code == 200,
///     "payload": {
///         features[0]: features[1]
///     }
/// });
/// ```
///
/// Trailing commas are allowed inside both arrays and objects.
///
/// ```
/// # use forest_ipld::ipld;
/// #
/// let value = ipld!([
///     "notice",
///     "the",
///     "trailing",
///     "comma -->",
/// ]);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! ipld {
    // Hide distracting implementation details from the generated rustdoc.
    ($($ipld:tt)+) => {
        ipld_internal!($($ipld)+)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! ipld_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: ipld_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        ipld_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        ipld_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!(false)] $($rest)*)
    };

    // * Next element is a `Link`.
    (@array [$($elems:expr,)*] Link($cid:expr) $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!(Link($cid))] $($rest)*)
    };

    // * Next element is `Bytes`.
    (@array [$($elems:expr,)*] Bytes($bz:expr) $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!(Bytes($bz))] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        ipld_internal!(@array [$($elems,)* ipld_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        ipld_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        ipld_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: ipld_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        ipld_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        ipld_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!(false)) $($rest)*);
    };

    // * Next value is a `Link`.
    (@object $object:ident ($($key:tt)+) (: Link($cid:expr) $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!(Link($cid))) $($rest)*);
    };

    // * Next value is `Bytes`
    (@object $object:ident ($($key:tt)+) (: Bytes($bz:expr) $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!(Bytes($bz))) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        ipld_internal!(@object $object [$($key)+] (ipld_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        ipld_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        ipld_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        ipld_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        ipld_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: ipld_internal!($($ipld)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::Ipld::Null
    };

    (true) => {
        $crate::Ipld::Bool(true)
    };

    (false) => {
        $crate::Ipld::Bool(false)
    };

    // * Cid link pattern
    (Link($cid:expr)) => {
        $crate::Ipld::Link($cid)
    };

    // * Bytes pattern
    (Bytes($bz:expr)) => {
        $crate::Ipld::Bytes($bz)
    };

    ([]) => {
        $crate::Ipld::List(ipld_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Ipld::List(ipld_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::Ipld::Map(::std::collections::BTreeMap::new())
    };

    ({ $($tt:tt)+ }) => {
        $crate::Ipld::Map({
            let mut object = ::std::collections::BTreeMap::new();
            ipld_internal!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::to_ipld(&$other).unwrap()
    };
}

// The ipld_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! ipld_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! ipld_unexpected {
    () => {};
}
