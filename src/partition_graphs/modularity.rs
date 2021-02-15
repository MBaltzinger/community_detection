use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use std::collections::HashMap;
use crate::partition_graphs::graph_ext::GraphExt;

fn subgraph_mod(graph: &UnGraph<(), f32>, q: &mut f32, m: f32, list_nodes: &Vec<NodeIndex>) {
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
            let a = k1 * k2 / m;
            match graph.find_edge(*node, *node2) {
                Some(e) => *q += graph.edge_weight(e)
                    .unwrap_or(&0.) - a,
                None => *q -= a,
            }
        }
    }
}

impl super::PartGraph {
    
    pub fn modularity(&self) -> f32 {
        let m = 2. * self.graph.sum_weights();
        let mut q = 0.;
        let mut k = self.partition.clone();
        k.sort();
        k.dedup();

        for p in k {
            let list_nodes = self.nodes_community(p); 
            subgraph_mod(&self.graph, &mut q, m, &list_nodes);
        }

        q / m
    }
    
    pub fn dq_remove(&self, node: &NodeIndex) -> f32 {
        let mut sum = 0.;
        let mut kiin = 0.; 
        let ki = self.graph.degree(&node);
        let m = self.graph.sum_weights();
        let community = self.get_community(node);
        for node2 in self.nodes_community(community) {
            sum += self.graph.degree(&node2);
            match self.graph.find_edge(*node, node2) {
                Some(e) => kiin += self.graph.edge_weight(e)
                    .unwrap_or(&0.),
                None => kiin += 0.,
                }
        } 
        match self.graph.find_edge(*node, *node) {
                Some(e) => return (kiin -ki*(sum-ki)/(2.*m) - self.graph.edge_weight(e).unwrap_or(&0.))/m,
                None =>  return (kiin -ki*(sum-ki)/(2.*m))/m,

            }
    }
    
    pub fn dq_add(&self, node: &NodeIndex, community: i32) -> f32 {
        let mut sum = 0.;
        let mut kiin = 0.; 
        let ki = self.graph.degree(&node);
        let m = self.graph.sum_weights();
        for node2 in self.nodes_community(community) {
            sum += self.graph.degree(&node2);
            match self.graph.find_edge(*node, node2) {
                Some(e) => kiin += self.graph.edge_weight(e)
                    .unwrap_or(&0.),
                None => kiin += 0.,
                }
        } 
        (kiin - (sum*ki)/(2.*m))/m
    }
    
    fn nodes_community(&self, community: i32) -> Vec<NodeIndex> {
        self.graph
        .node_indices()
        .filter(|n| self.partition[n.index()] == community)
        .collect()
    }
}