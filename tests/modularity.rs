use petgraph::graph::UnGraph;
use community_detection::partition_graphs::PartGraph;

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

#[test]
fn delta_modularity() {
    let graph = UnGraph::<(), f32>::from_edges(&[
        (0, 1, 1.),
        (2, 3, 10000.),
        (1, 2, 10000.),
        (1, 3, 10000.)
        ]);
    
    let partition = vec![0,0,1,1];
    
    let mut pgraph = PartGraph{
        graph: Box::new(graph),
        partition: partition
    };
    
    let first_m = pgraph.modularity();
    
    let mut node_iter = pgraph.graph.node_indices();
    node_iter.next();
    
    let node = node_iter.next().unwrap();
    let dq_add = pgraph.dq_add(&node, 1);
    let dq_remove = pgraph.dq_remove(&node);
    
    let partition = vec![0,1,1,1];
    pgraph.partition = partition; 
    let m = pgraph.modularity();

    let diff = dq_add - dq_remove - m +first_m;
    assert!(diff < 1e-4)
}