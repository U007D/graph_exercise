#![warn(clippy::all)]
#![forbid(overflowing_literals, unsafe_code)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used,)]
// ^^^ End of safety-critical lint section ^^^
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
// #![warn(clippy::multiple_crate_versions, clippy::print_on_stdout, clippy::unimplemented, clippy::use_debug)]
#![allow(clippy::match_bool,)]

mod error;
mod event;
mod event_manager_builder;
#[cfg(test)]
mod test_utils;
pub use self::{
    error::Error,
    event::{
        Event,
        fault::Fault,
        fault_level::FaultLevel,
        state::*,
    },
    event_manager_builder::EventManagerBuilder,
};
use std::result::Result as StdResult;
type Result<T> = StdResult<T, Error>;
