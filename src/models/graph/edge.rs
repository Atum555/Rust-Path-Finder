use crate::models::NodeId;

#[derive(Debug)]
pub struct Edge<T> {
    // Connections - IDs of connecting Nodes
    from: NodeId,
    to: NodeId,

    // Internal Edge data
    data: T,
}

impl<T> Edge<T> {
    pub fn new(from: NodeId, to: NodeId, data: T) -> Self {
        Self { from, to, data }
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
}
