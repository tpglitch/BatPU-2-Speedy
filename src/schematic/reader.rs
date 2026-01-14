use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_machine_code_lines(
    reader: BufReader<File>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| line.trim().to_string())
        .collect();

    while lines.len() < 1024 {
        lines.push("0000000000000000".to_string());
    }
    Ok(lines)
}
