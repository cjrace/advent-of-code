use std::io;

pub fn part_one(table: &Vec<Vec<String>>) -> io::Result<usize> {

    let mut safety_check: Vec<Vec<String>> = Vec::new();

    /* for every row in table, check if values are ascending by 1 2 or 3 when read from either left to right or right to left */
    for row in table {
        let mut ascending = true;
        let mut descending = true;

        for i in 0..row.len() - 1 {
            let current = row[i].parse::<i32>().unwrap();
            let next = row[i + 1].parse::<i32>().unwrap();

            if next - current != 1 && next - current != 2 && next - current != 3 {
                ascending = false;
            }

            if current - next != 1 && current - next != 2 && current - next != 3 {
                descending = false;
            }
        }

        if ascending || descending {
            safety_check.push(vec!["TRUE".to_string()]);
        } else {
            safety_check.push(vec!["FALSE".to_string()]);
        }
    }

    if table.len() != safety_check.len() {
        return Err(io::Error::new(io::ErrorKind::Other, "Table length does not match safety check length"));
    }

    let true_count = safety_check.iter().filter(|row| row[0] == "TRUE").count();

    Ok(true_count)
}
