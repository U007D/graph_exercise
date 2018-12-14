use crate::{
    Error,
    FaultDetector,
};
use super::*;

struct FaultDetectorFake {}

impl FaultDetectorFake {
    fn new() -> Self {
        Self {}
    }
}

impl FaultDetector for FaultDetectorFake {
    fn is_faulted(&self) -> bool {
        unimplemented!();
    }
}

#[test]
fn constructing_a_fault_manager_with_an_empty_graph_returns_an_error() {
    // given a FaultManagerBuilder instance
    let builder = FaultManagerBuilder::new();

    // when only .build() is called on the fault manager builder
    let result = builder.build();

    // then the result is an empty graph
    assert_eq!(result.unwrap_err(), Error::EmptyGraph);
}

#[test]
fn constructing_a_fault_manager_with_1_node_reports_1_node() {
    // given a FaultManagerBuilder instance
    let mut builder = FaultManagerBuilder::new();
    let mock = FaultDetectorFake::new();
    builder.add_detector(FaultDetectorFake);

    // when build() is called on the builder
    let result = builder.build();

    // then the result is a manager with one node
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn constructing_a_fault_manager_with_2_unconnected_nodes_reports_an_error() {
    // given a FaultManagerBuilder instance
    let mut builder = FaultManagerBuilder::new();
    builder.add_detector(());
    builder.add_detector(());
    builder.add_detector(());
    builder.add_detector(());
    builder.add_detector(());

    // when build() is called on the builder
    let result = builder.build();

    // then the result is a manager with one node
    assert_eq!(result.unwrap().len(), 5);
}

