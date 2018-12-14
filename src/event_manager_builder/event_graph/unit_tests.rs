use super::*;

#[test]
fn constructing_an_empty_graph_returns_a_len_of_0() {
    // given an empty fault manager graph
    let graph = EventGraph::new();

    // when its length is queried
    let result = graph.len();

    // then it should return 0
    assert_eq!(result, 0);

    // and is_empty() should be true
    assert!(graph.is_empty());
}

#[test]
fn constructing_an_empty_graph_returns_is_empty_true() {
    // given an empty fault manager graph
    let graph = EventGraph::new();

    // when it is queried for emptiness
    let result = graph.is_empty();

    // it should report true
    assert!(graph.is_empty());
}
