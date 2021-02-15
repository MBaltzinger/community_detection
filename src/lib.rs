use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

mod utils;

pub struct PartGraph {
    pub graph: Box<UnGraph<(), f32>>,
    pub partition: Vec<i32>, 
}

impl PartGraph {

    pub fn get_community(&self, node: &NodeIndex) -> i32 {
        self.partition[node.index()]
    }

    pub fn neighboring_communities(&self, node: &NodeIndex) -> Vec<i32> {
        let mut array_community: Vec<i32> = Vec::new();
        for neighbor in self.graph.neighbors(*node){
            let temp_community = self.get_community(&neighbor);
            if !array_community.contains(&temp_community){
               array_community.push(temp_community)
            }
        }
        array_community
    }

}