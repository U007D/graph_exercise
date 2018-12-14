mod fault_manager;
mod fault_manager_graph;
use crate::{
    FaultDetector,
    Result,
};
pub use self::{
    fault_manager::FaultManager,
    fault_manager_graph::{
        FaultManagerGraph,
        ReporterId,
    }
};
#[cfg(test)]
mod unit_tests;

pub struct FaultManagerBuilder<'a>{
    graph: FaultManagerGraph<'a>,
}

impl<'a> FaultManagerBuilder<'a> {
    pub fn new() -> Self {
        Self {
            graph: FaultManagerGraph::new(),
        }
    }

    pub fn add_detector(&mut self, detector: &'a dyn FaultDetector) -> ReporterId {
        self.graph.add_detector(detector)
    }

    pub fn build(self) -> Result<FaultManager<'a>> {
        FaultManager::new(self.graph)
    }
}
