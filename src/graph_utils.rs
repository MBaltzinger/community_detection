use petgraph::graph::UnGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

trait GraphExt {

    fn sum_weights(&self) -> f32;
    fn degree(&self, node : &NodeIndex) -> f32;
}

impl GraphExt for UnGraph<(), f32> {

    fn sum_weights(&self) -> f32 {
        let mut m : f32 = 0.; 
        for e in self.edge_indices() {
            match self.edge_weight(e) {
                Some(v) =>  m += *v,
                None => {}
            }
        }
        m
    }

    fn degree(&self, node: &NodeIndex) -> f32 {
        let mut k : f32 = 0.;
        for x in self.edges(*node) {
            k += x.weight();
        }
        k
    }
}

pub struct PartGraph {
    pub graph: Box<UnGraph<(), f32>>,
    pub partition: Vec<i32> 
}

impl PartGraph {

    pub fn modularity(&self) -> f32 {

        let m = 2. * self.graph.sum_weights();
        let mut q = 0.;
        let mut k = self.partition.clone();
        k.sort();
        k.dedup();

        for p in k {
            let list_nodes = self.graph.node_indices().filter(|n| self.partition[n.index()]==p).collect();
            subgraph_mod(&self.graph,&mut q , m, &list_nodes);
        }

        q/m
    }
}

fn subgraph_mod(graph: &UnGraph<(), f32>, q: &mut f32, m: f32, list_nodes : &Vec<NodeIndex>){

    let mut degree = HashMap::new();
    for node in list_nodes {
        let k1: f32;
        if degree.contains_key(node) {
            k1 = *degree.get(node).unwrap();
        } else {
            k1 = graph.degree(node);
            degree.insert(node, k1);
        }
        for node2 in list_nodes {
            let k2: f32;
            if degree.contains_key(node2) {
                k2 = *degree.get(node2).unwrap();
            } else {
                k2 = graph.degree(node2);
                degree.insert(node2, k2);
            }
            let a = k1*k2/m;
            match graph.find_edge(*node,*node2) {
                Some(e) =>  *q += graph.edge_weight(e).unwrap_or(&0.) - a,
                None => *q -= a
            }
        }
    }
}