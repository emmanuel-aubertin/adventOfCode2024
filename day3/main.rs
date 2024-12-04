use regex::Regex;
use std::fs::read_to_string;

// Extract from documentation
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn compute_corrupted_memory(lines: Vec<String>) -> i128 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for exp in lines {
        for cap in re.captures_iter(&exp) {
            let x: i128 = cap[1].parse().unwrap();
            let y: i128 = cap[2].parse().unwrap();
            total += x * y;
        }
    }
    total
}

fn main() -> std::io::Result<()> {
    let file = "data.txt";
    let data: Vec<String> = read_lines(file);
    let  result: i128 = compute_corrupted_memory(data);

    println!("Result part one: {}", result);
    Ok(())
}
