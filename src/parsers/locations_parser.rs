use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::models::Location;

pub fn parse_file(file: &File) -> Result<Vec<Location>, Box<dyn Error>> {
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

fn parse_line(line: &String) -> Result<Location, Box<dyn Error>> {
    let mut fields = line.split(',');

    let location = fields.next().ok_or("Missing location")?.trim();
    let id = fields.next().ok_or("Missing id")?.trim();
    let code = fields.next().ok_or("Missing code")?.trim();
    let parking: bool = match fields.next().ok_or("Missing parking")?.trim() {
        "1" => true,
        "0" => false,
        _ => return Err("Invalid parking value".into()),
    };

    Ok(Location::new(id, code, parking, location))
}
