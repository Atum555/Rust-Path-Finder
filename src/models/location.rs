#[derive(Debug)]
pub struct Location {
    id: u64,
    code: String,
    parking: bool,
    location: String,
}

impl Location {
    pub fn new(id: u64, code: String, parking: bool, location: String) -> Self {
        Self {
            id,
            code,
            parking,
            location,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn parking(&self) -> bool {
        self.parking
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}
