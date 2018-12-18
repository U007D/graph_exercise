mod error;
mod event;
mod event_manager_builder;
#[cfg(test)]
mod test_utils;
pub use self::{
    error::Error,
    event::{
        Event,
        fault_descriptor::*,
        state_descriptor::*,
    },
    event_manager_builder::EventManagerBuilder,
};
use std::result::Result as StdResult;
type Result<T> = StdResult<T, Error>;
