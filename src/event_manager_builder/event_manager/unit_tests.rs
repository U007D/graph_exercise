use super::*;

#[test]
fn constructing_an_event_manager_with_an_empty_graph_returns_an_error() {
    // given an empty graph
    let graph = EventManagerGraph::new();

    // when constructing a EventManager with it
    let result = EventManager::new(graph);

    // then the result should be an error
    assert_eq!(result.unwrap_err(), Error::EmptyGraph);
}
