use std::fs::read_to_string;

// Extract from documentation
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_file(file: &str) -> Vec<Vec<u32>> {
    let lines = read_lines(file);
    let mut result: Vec<Vec<u32>> = Vec::new();
    result.push(Vec::new());
    result.push(Vec::new());
    for line in lines {
        let parts: Vec<_> = line.trim().split("   ").collect();
        result[0].push(parts[0].parse::<u32>().unwrap());
        result[1].push(parts[1].parse::<u32>().unwrap());
    }
    result
}

fn sort_vect(vect: Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = vect.clone();
    for n in 0..result.len(){
        let mut swapped: bool = false;
        for i in 0..(result.len() - 1 - n)  {
            if result[i] > result[i + 1] {
                let temp: u32 = result[i];
                result[i] = result[i + 1];
                result[i + 1] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    result
}

fn compute_distance(a: Vec<u32>, b: Vec<u32>) -> i64 { 
    let mut result: i64 = 0;
    for i in 0..a.len() {
        let temp_a: i64 = a[i].into();
        let temp_b: i64 = b[i].into();
        let temp: i64 = temp_a - temp_b;
        result += temp.abs();
    }
    result
}

fn main() -> std::io::Result<()> {
    let file = "data.txt";
    let mut data: Vec<Vec<u32>> = parse_file(file);

    data[0] = sort_vect(data[0].clone());
    data[1] = sort_vect(data[1].clone());

    println!("Distance: {}", compute_distance(data[0].clone(), data[1].clone()));


    Ok(())
}
