use std::{fmt::Display, ops::AddAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EdgeId(pub u64);

impl std::fmt::Debug for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // always print inline, even if f.alternate() is true
        write!(f, "EdgeId({})", self.0)
    }
}

impl Display for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for EdgeId {
    fn from(id: u64) -> Self {
        EdgeId(id)
    }
}

impl From<EdgeId> for u64 {
    fn from(edge_id: EdgeId) -> Self {
        edge_id.0
    }
}

impl AddAssign<u64> for EdgeId {
    fn add_assign(&mut self, other: u64) {
        self.0 += other;
    }
}
