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


fn compute_corrupted_memory_with_conditions(lines: Vec<String>) -> i128 {
    // Regex patterns to match operations
    let re_op = Regex::new(r"do\(\)|don't\(\)|mul\s*[\(\[\{][^)\]\}]*[\)\]\}]").unwrap();
    let re_mul = Regex::new(r"mul\s*[\(\[\{]\s*(\d+)\s*[,;:\s]\s*(\d+)\s*[\)\]\}]").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
        
    let mut total = 0;
    let mut mul_enabled = true;
   
    for line in lines {
        // Find all operation matches in the line
        let mut matches = Vec::new();
        for mat in re_op.find_iter(&line) {
            matches.push((mat.start(), mat.as_str().to_string()));
        }

        // Process matches in the order they appear
        matches.sort_by_key(|&(pos, _)| pos);

        for (_, m) in matches {
            if re_do.is_match(&m) {
                mul_enabled = true;
            } else if re_dont.is_match(&m) {
                mul_enabled = false;
            } else if mul_enabled {
                if let Some(caps) = re_mul.captures(&m) {
                    let a: i128 = caps.get(1).unwrap().as_str().parse().unwrap();
                    let b: i128 = caps.get(2).unwrap().as_str().parse().unwrap();
                    total += a * b;
                }
            }
        }
    }
    total
}



fn main() -> std::io::Result<()> {
    let file = "data.txt";
    let data: Vec<String> = read_lines(file);

    println!("Result part one: {}", compute_corrupted_memory(data.clone()));

    println!("Result part one: {}", compute_corrupted_memory_with_conditions(data.clone()));

    Ok(())
}
