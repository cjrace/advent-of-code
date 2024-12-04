mod functions;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    /* read file line by line into a table */
    let path = Path::new("day2_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut table: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        table.push(row);
    }

    let part_one_response = functions::part_one(&table)?;

    println!("Part one answer: {:?}", part_one_response);
    
    Ok(())
}
