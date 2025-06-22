use crate::models::{Distance, Location};
use sdw_graph::{Graph, NodeId};

pub trait GraphExt {
    fn get_node_id(&self, identifier: &str) -> Option<NodeId>;
}

impl GraphExt for Graph<Location, Distance> {
    fn get_node_id(&self, identifier: &str) -> Option<NodeId> {
        for node in self.nodes() {
            if node.1.data().id() == identifier {
                return Some(*node.0);
            }
        }

        for node in self.nodes() {
            if node.1.data().code() == identifier {
                return Some(*node.0);
            }
        }

        None
    }
}
