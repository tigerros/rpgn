#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
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

dry_mods::mods! {
    mod pub use date,
    eco,
    eco_category,
    move_number,
    movetext,
    outcome,
    pgn,
    raw_header_owned,
    round;
}

/// These are samples I use in tests and benchmarks.
/// They're public because benchmarks get the same crate you get.
pub mod samples;
mod visitor;
