mod event_manager;
mod event_manager_graph;
use crate::{
    Event,
    Result,
};
pub use self::{
    event_manager::EventManager,
    event_manager_graph::{
        EventManagerGraph,
        EventId,
    }
};
#[cfg(test)]
mod unit_tests;

pub struct EventManagerBuilder<'a>{
    graph: EventManagerGraph<'a>,
}

impl<'a> EventManagerBuilder<'a> {
    pub fn new() -> Self {
        Self {
            graph: EventManagerGraph::new(),
        }
    }

    pub fn add_event(&mut self, event: &'a dyn Event) -> EventId {
        self.graph.add_event(event)
    }

    pub fn build(self) -> Result<EventManager<'a>> {
        EventManager::new(self.graph)
    }
}
