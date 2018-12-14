use super::*;

#[test]
fn an_empty_graph_returns_a_len_of_0() {
    // given an empty fault manager graph
    let graph = FaultManagerGraph::new();

    // when its length is queried
    let result = graph.len();

    // then it should return 0
    assert_eq!(result, 0);
}
