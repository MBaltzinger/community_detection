use petgraph::graph::UnGraph;
use community_detection::graph_utils::PartGraph;

#[test]
fn modularity() {
    let graph = UnGraph::<(), f32>::from_edges(&[
        (0, 1, 10000.),
        (2, 3, 10000.),
        (1, 2, 1.)
        ]);
    
    let partition = vec![0,0,1,1];
    
    let pgraph = PartGraph{
        graph: Box::new(graph),
        partition: partition
    };

    let m = pgraph.modularity();
    assert_eq!(m, 0.49995);
}