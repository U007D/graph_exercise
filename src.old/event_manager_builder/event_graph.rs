#[cfg(test)]
mod unit_tests;
use crate::Event;
use petgraph::{
    Directed,
    graph::{
        Graph,
        NodeIndex,
    },
};
use std::fmt::{
    Debug,
    Formatter,
    Result as FmtResult,
};
type GraphType<'a> = Graph<&'a dyn Event, (), Directed, usize>;

#[derive(Debug)]
pub struct EventId(NodeIndex<usize>);

pub struct EventGraph<'a>(GraphType<'a>);

impl<'a> EventGraph<'a> {
    pub(super) fn new() -> Self {
        Self(GraphType::<'a>::default())
    }

    pub(super) fn add_event(&mut self, event: &'a dyn Event) -> EventId { EventId(self.0.add_node(event)) }

    #[inline]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    #[inline]
    pub fn len(&self) -> usize { self.0.node_count() }
}

impl<'a> Debug for EventGraph<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // TODO(anyone): Write graph dump function
        write!(f, "<EventGraph>")
    }
}
