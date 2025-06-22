use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::models::Location;
use crate::parsers::{Field, ParseError, ParseError::*};

pub fn parse_file(file: &File) -> Result<Vec<Location>, ParseError> {
    let mut locations = Vec::new();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Skip the first line
    lines.next();

    for line in lines {
        let line = line?;
        let location = parse_line(&line)?;
        locations.push(location);
    }

    Ok(locations)
}

fn parse_line(line: &String) -> Result<Location, ParseError> {
    let mut fields = line.split(',');

    let location = fields.next().ok_or(NoField(Field::Location))?.trim();
    let id = fields.next().ok_or(NoField(Field::Id))?.trim();
    let code = fields.next().ok_or(NoField(Field::Code))?.trim();
    let parking = match fields.next().ok_or(NoField(Field::Parking))?.trim() {
        "1" => true,
        "0" => false,
        value => return Err(InvalidValue(Field::Parking, value.to_string())),
    };

    Ok(Location::new(id, code, parking, location))
}
