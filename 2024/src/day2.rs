use std::fs;

fn check_sequence(parts: &[u32]) -> bool {
    let increasing: bool = parts[1] > parts[0];
    
    if increasing {
        for i in 1..parts.len() {
            if parts[i] <= parts[i - 1] {
                println!("{} <= {}", parts[i], parts[i - 1]);
                return false;
            }
        }
    } else { // decreasing
        for i in 1..parts.len() {
            if parts[i] >= parts[i - 1] {
                println!("{} >= {}", parts[i], parts[i - 1]);
                return false;
            }
        }
    }

    // check non-too-big-increment
    for i in 1..parts.len() {
        let diff: i32 = (parts[i - 1] as i32 - parts[i] as i32).abs();
        if diff > 3 {
            println!("{} - {} = {}", parts[i], parts[i - 1], diff);
            return false;
        }
    }

    return true;
}

pub fn solve() {
    let file_path = "src/2024_02.txt";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            
            let mut safe_reports: usize =
                contents
                .lines()
                .filter(|line| {
                    let parts: Vec<u32> = line
                        .split(" ")
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    return check_sequence(&parts);
                })
                .count();
            println!("Day 2.1: {}", safe_reports);

            safe_reports =
                contents
                .lines()
                .filter(|line| {
                    let parts: Vec<u32> = line
                        .split(" ")
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    let normal_check: bool = check_sequence(&parts);
                    
                    if(normal_check) { // safe from beginning
                        return true;
                    }

                    // alter levels
                    for i in 0..parts.len() {
                        let mut mod_parts: Vec<u32> = parts.clone();
                        mod_parts.remove(i);
                        let altered_check: bool = check_sequence(&mod_parts);
                        if(altered_check) { // safe after modding
                            return true;
                        }
                    }

                    return false;
                })
                .count();
            println!("Day 2.2: {}", safe_reports);
        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }
}