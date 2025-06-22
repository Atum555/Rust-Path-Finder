use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::models::Distance;
use crate::parsers::{Field, ParseError, ParseError::*};

pub fn parse_file(file: &File) -> Result<Vec<(String, String, Distance)>, ParseError> {
    let mut distances = Vec::new();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Skip the first line
    lines.next();

    for line in lines {
        let line = line?;
        let (code1, code2, distance) = parse_line(&line)?;
        distances.push((code1, code2, distance));
    }

    Ok(distances)
}

fn parse_line(line: &String) -> Result<(String, String, Distance), ParseError> {
    let mut fields = line.split(',');

    let code1 = fields
        .next()
        .ok_or(NoField(Field::Code))?
        .trim()
        .to_string();
    let code2 = fields
        .next()
        .ok_or(NoField(Field::Code))?
        .trim()
        .to_string();
    let driving = fields
        .next()
        .ok_or(NoField(Field::Driving))?
        .trim()
        .parse::<u64>()
        .ok();
    let walking = fields.next().ok_or(NoField(Field::Walking))?.trim();
    let walking = walking
        .parse::<u64>()
        .or_else(|_| Err(InvalidValue(Field::Walking, walking.to_string())))?;

    Ok((code1, code2, Distance::new(walking, driving)))
}
