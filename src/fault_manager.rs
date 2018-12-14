#[cfg(test)]
mod unit_tests;
use crate::{
    Error,
    FaultManagerGraph,
    Result,
};

#[derive(Debug, PartialEq)]
pub struct FaultManager {}

impl FaultManager {
    pub fn new(_graph: FaultManagerGraph) -> Result<Self> {
        Err(Error::EmptyGraph)
    }
}