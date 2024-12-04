use std::io;

pub struct Table {
    data: Vec<Vec<String>>,
}

impl Table {
    pub fn new(data: Vec<Vec<String>>) -> Self {
        Table { data }
    }

    pub fn part_one(&self) -> io::Result<usize> {
        let mut safety_check: Vec<Vec<String>> = Vec::new();

        for row in &self.data {
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

        if self.data.len() != safety_check.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "Table length does not match safety check length"));
        }

        let true_count = safety_check.iter().filter(|row| row[0] == "TRUE").count();
        Ok(true_count)
    }

    pub fn part_two(&self) -> io::Result<usize> {
        let mut dampened_safety_check: Vec<Vec<String>> = Vec::new();

        for row in &self.data {
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
            if !ascending && !descending {
                for j in 0..row.len() {
                    let mut temp_row = row.clone();
                    temp_row.remove(j);

                    ascending = true;
                    descending = true;

                    for k in 0..temp_row.len() - 1 {
                        let current = temp_row[k].parse::<i32>().unwrap();
                        let next = temp_row[k + 1].parse::<i32>().unwrap();

                        if next - current != 1 && next - current != 2 && next - current != 3 {
                            ascending = false;
                        }

                        if current - next != 1 && current - next != 2 && current - next != 3 {
                            descending = false;
                        }
                    }

                    if ascending || descending {
                        break;
                    }
                }
            }
            }

            if ascending || descending {
                dampened_safety_check.push(vec!["TRUE".to_string()]);
            } else {
                dampened_safety_check.push(vec!["FALSE".to_string()]);
            }
        }

        if self.data.len() != dampened_safety_check.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "Table length does not match dampened safety check length"));
        }

        let safe_count = dampened_safety_check.iter().filter(|row| row[0] == "TRUE").count();
        Ok(safe_count)
    }
}