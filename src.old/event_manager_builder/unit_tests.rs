#![allow(clippy::result_unwrap_used)]
use crate::{
    Error,
    Event,
    test_utils::*,
};
use super::*;
struct EventFake {}

impl EventFake {
    fn new() -> Self {
        Self {}
    }
}

impl Event for EventFake {
    fn is_triggered(&self) -> bool {
        unimplemented!();
    }
}

#[test]
fn constructing_an_event_manager_with_an_empty_graph_returns_an_error() {
    // given a EventManagerBuilder instance
    let builder = EventManagerBuilder::new();

    // when only .build() is called on the event manager builder
    let result = builder.build();

    // then the result is an empty graph
    assert_eq!(result.unwrap_err(), Error::EmptyGraph);
}

#[test]
fn constructing_an_event_manager_with_1_node_reports_2_nodes() {
    // given a EventManagerBuilder instance
    let mut builder = EventManagerBuilder::new();
    let mock_event = EventFake::new();
    builder.add_event(&mock_event);

    // when build() is called on the builder
    let result = builder.build();

    // then the result is a manager with one node
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn constructing_an_event_manager_with_5_unconnected_nodes_reports_ok() {
    // given a EventManagerBuilder instance
    let mut builder = EventManagerBuilder::new();
    let mock_event = EventFake::new();
    builder.add_event(&mock_event);
    builder.add_event(&mock_event);
    builder.add_event(&mock_event);
    builder.add_event(&mock_event);
    builder.add_event(&mock_event);

    // when build() is called on the builder
    let result = builder.build();

    // then the result is a manager with six nodes
    assert_eq!(result.unwrap().len(), 6);
}

#[test]
fn event_manager_builder_connects_events() {
    // given an event manager graph
    let mut builder = EventManagerBuilder::new();
    let source_1 = TempSensorMock::new(ShouldGenerateFault::No);
    let source_2 = TempSensorMock::new(ShouldGenerateFault::No);
    let event_id1 = builder.add_event(&source_1);
    let event_id2 = builder.add_event(&source_2);

    // when edge is added between two nodes
    let result = builder.add_edges(&[(event_id1, event_id2)]);

    // it should succeed
    assert_eq!(result.is_ok(), true);
}

#[test]
fn event_manager_builder_builds_connected_event_graph() {
    // given an event manager graph
    let mut builder = EventManagerBuilder::new();
    let source_1 = TempSensorMock::new(ShouldGenerateFault::No);
    let source_2 = TempSensorMock::new(ShouldGenerateFault::No);
    let event_id1 = builder.add_event(&source_1);
    let event_id2 = builder.add_event(&source_2);
    builder.add_edges(&[(event_id1, event_id2)]).unwrap();

    // when EventManager is constructed
    let result = builder.build();

    // it should build successfully
    assert_eq!(result.unwrap().len(), 2);

}


