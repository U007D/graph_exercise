mod event_graph;
mod event_manager;
use crate::{
    Event,
    Result,
};
pub use self::{
    event_manager::EventManager,
    event_graph::{
        EventGraph,
        EventId,
    },
};
#[cfg(test)]
mod unit_tests;

pub struct EventManagerBuilder<'a> {
    graph: EventGraph<'a>,
}

impl<'a> EventManagerBuilder<'a> {
    pub fn new() -> Self {
        Self {
            graph: EventGraph::new(),
        }
    }

    pub fn add_edges(&mut self, node_pairs: &[(EventId, EventId)]) -> Result<&mut Self> {
        Ok(self)
    }

    pub fn add_event(&mut self, event: &'a dyn Event) -> EventId {
        self.graph.add_event(event)
    }

    pub fn build(self) -> Result<EventManager<'a>> {

        let summary_node_id = self.graph.add_event(summary_event)
            EventManager::new(self.graph) // sort
    }
}

impl<'a> Default for EventManagerBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
