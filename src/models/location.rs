#[derive(Debug)]
pub struct Location {
    id: String,
    code: String,
    parking: bool,
    location: String,
}

impl Location {
    pub fn new(id: &str, code: &str, parking: bool, location: &str) -> Self {
        Self {
            id: id.to_string(),
            code: code.to_string(),
            parking,
            location: location.to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
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
