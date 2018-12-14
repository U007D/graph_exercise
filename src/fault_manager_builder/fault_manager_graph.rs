#[cfg(test)]
mod unit_tests;
use crate::FaultDetector;
use petgraph::{
    Directed,
    graph::{
        Graph,
        NodeIndex,
    },
};
type GraphType<'a> = Graph<&'a dyn FaultDetector, (), Directed, usize>;

#[derive(Debug)]
pub struct ReporterId(NodeIndex<usize>);

#[derive(Debug)]
pub struct FaultManagerGraph<'a>(GraphType<'a>);

impl<'a> FaultManagerGraph<'a> {
    pub(super) fn new() -> Self {
        Self(GraphType::<'a>::default())
    }

    pub(super) fn add_detector(&mut self, detector: &'a dyn FaultDetector) -> ReporterId {
        ReporterId(self.0.add_node(detector))
    }

    #[inline]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    #[inline]
    pub fn len(&self) -> usize { self.0.node_count() }
}

