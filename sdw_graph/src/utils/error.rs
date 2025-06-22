use crate::{EdgeId, NodeId};

#[derive(Debug)]
pub enum GraphError {
    NodeNotFound(NodeId),
    EdgeAlreadyExists {
        from_id: NodeId,
        to_id: NodeId,
        edge_id: EdgeId,
    },
}
