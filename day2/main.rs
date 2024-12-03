use std::fs::read_to_string;

// Extract from documentation
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_file(file: &str) -> Vec<Vec<i64>> {
    let lines = read_lines(file);
    let mut result: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let parts: Vec<_> = line.trim().split_whitespace().collect();
        let row: Vec<i64> = parts
            .iter()
            .map(|part| part.parse::<i64>().unwrap())
            .collect();
        result.push(row);
    }
    result
}

fn compute_safety_with_dampener(list: Vec<Vec<i64>>) -> i64 {
    let mut safety: i64 = 0;
    for row in &list {
        if is_row_safe_with_dampener(row) {
            safety += 1;
        }
    }
    safety
}

fn is_row_safe_with_dampener(row: &Vec<i64>) -> bool {
    if is_row_safe(row) {
        return true;
    }

    for i in 0..row.len() {
        let mut modified_row = row.clone();
        modified_row.remove(i);
        if is_row_safe(&modified_row) {
            return true;
        }
    }

    false
}

fn is_row_safe(row: &Vec<i64>) -> bool {
    let mut is_increase = true;
    if row.len() < 2 {
        return true; // Single-element rows are safe
    }

    if row[0] < row[1] {
        is_increase = false;
    }

    for i in 0..row.len() - 1 {
        if is_increase {
            if row[i] - row[i + 1] < 1 || row[i] - row[i + 1] > 3 {
                return false;
            }
        } else {
            if row[i + 1] - row[i] < 1 || row[i + 1] - row[i] > 3 {
                return false;
            }
        }
    }
    true
}

fn main() -> std::io::Result<()> {
    let file = "data.txt";
    let data: Vec<Vec<i64>> = parse_file(file);


    println!("Part one : {:?}", compute_safety(data.clone()));
    println!("Part two (with dampener): {:?}", compute_safety_with_dampener(data.clone()));

    Ok(())
}
