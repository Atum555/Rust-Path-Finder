use std::{fmt::Display, ops::AddAssign};

#[derive(Debug)]
pub enum GraphError {
    NodeNotFound(NodeId),
    EdgeAlreadyExists {
        from_id: NodeId,
        to_id: NodeId,
        edge_id: EdgeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub u64);

impl Display for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for NodeId {
    fn from(id: u64) -> Self {
        NodeId(id)
    }
}

impl From<u64> for EdgeId {
    fn from(id: u64) -> Self {
        EdgeId(id)
    }
}

impl From<NodeId> for u64 {
    fn from(node_id: NodeId) -> Self {
        node_id.0
    }
}

impl From<EdgeId> for u64 {
    fn from(edge_id: EdgeId) -> Self {
        edge_id.0
    }
}

impl AddAssign<u64> for NodeId {
    fn add_assign(&mut self, other: u64) {
        self.0 += other;
    }
}

impl AddAssign<u64> for EdgeId {
    fn add_assign(&mut self, other: u64) {
        self.0 += other;
    }
}
