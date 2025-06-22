use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing field: {0}")]
    NoField(Field),

    #[error("Invalid value for field {0}: {1}")]
    InvalidValue(Field, String),
}

#[derive(Debug)]
pub enum Field {
    Id,
    Code,
    Parking,
    Location,
    Driving,
    Walking
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_name = match self {
            Field::Id => "Id",
            Field::Code => "Code",
            Field::Parking => "Parking",
            Field::Location => "Location",
        };
        write!(f, "{}", field_name)
    }
}
