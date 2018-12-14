mod error;
mod fault;
mod fault_manager_builder;
pub use self::{
    error::Error,
    fault::{
        FaultDetector,
        TempFaultDetector,
    },
    fault_manager_builder::FaultManagerBuilder,
};
use std::result::Result as StdResult;
type Result<T> = StdResult<T, Error>;
