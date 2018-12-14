#[cfg(test)]
mod unit_tests;
use crate::{
    Error,
    Result,
};
use super::FaultManagerGraph;

#[derive(Debug)]
pub struct FaultManager<'a> {
    graph: FaultManagerGraph<'a>,
}

impl<'a> FaultManager<'a> {
    pub(super) fn new(graph: FaultManagerGraph<'a>) -> Result<Self> {
        match graph.is_empty() {
            true => Err(Error::EmptyGraph),
            false => Ok(Self {
                graph,
            })
        }
    }

    #[inline]
    pub fn len(&self) -> usize { self.graph.len() }
}
