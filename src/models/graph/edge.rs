use crate::models::{EdgeId, NodeId};

#[derive(Debug)]
pub struct Edge<T> {
    // Connections - IDs of connecting Nodes
    from: NodeId,
    to: NodeId,

    // Internal Edge data
    data: T,

    // Reverse Edge
    reverse: Option<EdgeId>,
}

impl<T> Edge<T> {
    pub fn new(from: NodeId, to: NodeId, data: T) -> Self {
        Self {
            from,
            to,
            data,
            reverse: None,
        }
    }

    pub fn from(&self) -> NodeId {
        self.from
    }

    pub fn to(&self) -> NodeId {
        self.to
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn reverse(&self) -> Option<EdgeId> {
        self.reverse
    }

    pub fn set_reverse(&mut self, reverse: Option<EdgeId>) {
        self.reverse = reverse;
    }
}
