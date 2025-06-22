use std::{fmt::Display, ops::AddAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeId(pub u64);

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // always print inline, even if f.alternate() is true
        write!(f, "NodeId({})", self.0)
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for NodeId {
    fn from(id: u64) -> Self {
        NodeId(id)
    }
}

impl From<NodeId> for u64 {
    fn from(node_id: NodeId) -> Self {
        node_id.0
    }
}

impl AddAssign<u64> for NodeId {
    fn add_assign(&mut self, other: u64) {
        self.0 += other;
    }
}
