mod functions;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use functions::Table;

fn main() -> io::Result<()> {
    /* read file line by line into a table */
    let path = Path::new("day2_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut data: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        data.push(row);
    }

    let table = Table::new(data);

    let part_one_response = table.part_one()?;

    println!("Part one answer: {:?}", part_one_response);
    
    let part_two_response = table.part_two()?;

    println!("Part two answer: {:?}", part_two_response);

    Ok(())
}
