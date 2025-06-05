#[derive(Debug)]
pub struct Distance {
    walking: u64,
    driving: Option<u64>,
}

impl Distance {
    pub fn new(walking: u64, driving: Option<u64>) -> Self {
        Self { walking, driving }
    }

    pub fn walking(&self) -> u64 {
        self.walking
    }

    pub fn driving(&self) -> Option<u64> {
        self.driving
    }
}
