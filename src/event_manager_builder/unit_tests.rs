use crate::{
    Error,
    Event,
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

    // when only .build() is called on the fault manager builder
    let result = builder.build();

    // then the result is an empty graph
    assert_eq!(result.unwrap_err(), Error::EmptyGraph);
}

#[test]
fn constructing_an_event_manager_with_1_node_reports_1_node() {
    // given a EventManagerBuilder instance
    let mut builder = EventManagerBuilder::new();
    let mock_event = EventFake::new();
    builder.add_event(&mock_event);

    // when build() is called on the builder
    let result = builder.build();

    // then the result is a manager with one node
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn constructing_an_event_manager_with_2_unconnected_nodes_reports_an_error() {
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

    // then the result is a manager with one node
    assert_eq!(result.unwrap().len(), 5);
}

