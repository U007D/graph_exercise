use super::*;

#[test]
fn constructing_a_fault_manager_with_an_empty_graph_returns_an_error() {
    // given an empty graph
    let graph = FaultManagerGraph::new();

    // when constructing a FaultManager with it
    let result = FaultManager::new(graph);

    // then the result should be an error
    assert_eq!(result.unwrap_err(), Error::EmptyGraph);
}
