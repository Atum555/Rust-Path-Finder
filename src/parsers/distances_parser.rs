use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::models::Distance;

pub fn parse_file(file: &File) -> Result<Vec<(String, String, Distance)>, Box<dyn Error>> {
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

fn parse_line(line: &String) -> Result<(String, String, Distance), Box<dyn Error>> {
    let mut fields = line.split(',');

    let code1 = fields
        .next()
        .ok_or("Missing first location code")?
        .trim()
        .to_string();
    let code2 = fields
        .next()
        .ok_or("Missing second location code")?
        .trim()
        .to_string();
    let driving = fields
        .next()
        .ok_or("Missing driving time")?
        .trim()
        .parse::<u64>()
        .ok();
    let walking = fields
        .next()
        .ok_or("Missing driving time")?
        .trim()
        .parse::<u64>()?;

    Ok((code1, code2, Distance::new(walking, driving)))
}
