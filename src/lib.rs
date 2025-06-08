#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::literal_string_with_formatting_args)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::must_use_candidate)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![allow(clippy::module_name_repetitions)]
#![warn(
    clippy::arithmetic_side_effects,
    clippy::unreachable,
    clippy::unchecked_duration_subtraction,
    clippy::todo,
    clippy::string_slice,
    clippy::panic_in_result_fn,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::exit,
    clippy::as_conversions,
    clippy::large_futures,
    clippy::large_stack_arrays,
    clippy::large_stack_frames,
    clippy::modulo_one,
    clippy::mem_replace_with_uninit,
    clippy::iterator_step_by_zero,
    clippy::invalid_regex,
    clippy::print_stdout,
    clippy::print_stderr
)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod visitor;

dry_mods::mods! {
    pub mod date, round;
    mod pub use
    move_number,
    outcome,
    pgn,
    raw_header_owned,
    movetext;
}

pub use date::Date;
pub use round::Round;

/// These are samples I use in tests and benchmarks.
/// They're public because benchmarks get the same crate you get.
#[doc(hidden)]
pub mod samples;
pub use movetext::Movetext;

/// Create a [`Variation`](crate::movetext::Variation) out of SAN literals.
///
/// # Syntax
/// See the `samples.rs` file in the repository.
///
/// # Panics
/// See [`SanPlus::from_ascii`](shakmaty::san::SanPlus::from_ascii).
#[macro_export]
macro_rules! variation {
    (_turn: $san:literal) => {
        $crate::movetext::SanWithVariations { san: ::shakmaty::san::SanPlus::from_ascii($san).unwrap(), variations: vec![] }
    };
    (_turn: ($san:literal, [$($vars:tt),+])) => {
        $crate::movetext::SanWithVariations { san: ::shakmaty::san::SanPlus::from_ascii($san).unwrap(), variations: vec![$($crate::variation! $vars),+] }
    };
    {$($turn:tt),+} => {
        $crate::movetext::Variation(vec![$($crate::variation!(_turn: $turn)),+])
    };
}

/// Create a [`Sans`](crate::movetext::Sans) out of a list SAN literals.
///
/// # Panics
/// See [`SanPlus::from_ascii`](shakmaty::san::SanPlus::from_ascii).
#[macro_export]
macro_rules! sans {
    ($($san:literal),+) => {
        $crate::movetext::Sans(vec![$(::shakmaty::san::SanPlus::from_ascii($san).unwrap()),+])
    };
}

#[cfg(feature = "serde")]
macro_rules! serde_display_from_str {
    ($type:ident) => {
        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.to_string().as_str())
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error;
                <&str>::deserialize(deserializer)?
                    .parse()
                    .map_err(D::Error::custom)
            }
        }
    };

    ($type:ident<$g:ident: Display + FromStr>) => {
        impl<$g> serde::Serialize for $type<$g>
        where
            $g: std::fmt::Display + std::str::FromStr,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.to_string().as_str())
            }
        }

        impl<'de, $g> serde::Deserialize<'de> for $type<$g>
        where
            $g: std::fmt::Display + std::str::FromStr,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <&str>::deserialize(deserializer)?
                    .parse()
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

#[cfg(feature = "serde")]
pub(crate) use serde_display_from_str;
