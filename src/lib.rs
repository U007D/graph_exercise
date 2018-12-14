mod error;
mod fault_manager;
mod fault_manager_graph;
pub use crate::{
    error::Error,
    fault_manager::FaultManager,
    fault_manager_graph::FaultManagerGraph,
};
use std::result::Result as StdResult;
type Result<T> = StdResult<T, Error>;