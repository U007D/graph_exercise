#[cfg(test)]
mod unit_tests;
use crate::{
    Error,
    Result,
};
use super::EventGraph;

#[derive(Debug)]
pub struct EventManager<'a> {
    graph: EventGraph<'a>,
}

impl<'a> EventManager<'a> {
    pub(super) fn new(graph: EventGraph<'a>) -> Result<Self> {
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
