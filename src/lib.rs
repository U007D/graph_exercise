mod error;
mod event;
mod event_manager_builder;
pub use crate::{
    error::Error,
    event::{
        Event,
        TempFaultDetector,
    },
    event_manager_builder::EventManagerBuilder,
};
use std::result::Result as StdResult;
type Result<T> = StdResult<T, Error>;
