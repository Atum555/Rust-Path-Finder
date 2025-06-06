use std::collections::HashMap;

use crate::models::{EdgeId, NodeId};

#[derive(Debug)]
pub struct Node<T> {
    // Internal Node data
    data: T,

    // Connections - Map:
    // Key -> Connecting Node ID
    // Value -> Edge ID
    pub incoming: HashMap<NodeId, EdgeId>,
    pub outgoing: HashMap<NodeId, EdgeId>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: data,
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}
