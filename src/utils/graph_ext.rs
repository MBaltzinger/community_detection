use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

pub trait GraphExt {
   fn sum_weights(&self) -> f32;
   fn degree(&self, node: &NodeIndex) -> f32;
}

impl GraphExt for UnGraph<(), f32> {
    fn sum_weights(&self) -> f32 {
        let mut m: f32 = 0.;
        for e in self.edge_indices() {
            match self.edge_weight(e) {
                Some(v) => m += *v,
                None => {}
            }
        }
        m
    }

    fn degree(&self, node: &NodeIndex) -> f32 {
        let mut k: f32 = 0.;
        for x in self.edges(*node) {
            k += x.weight();
        }
        k
    }
}
