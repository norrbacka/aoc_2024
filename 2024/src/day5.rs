use std::fs;
use regex::Regex;

pub fn solve() {
    let file_path = "src/2024_05.txt";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            
            let mut rules: Vec<(i32, i32)> = Vec::new();
            let mut updates: Vec<Vec<i32>> = Vec::new(); 

            for line in contents.lines() {
                if line.contains("|") {
                    let parts: Vec<&str> = line.split("|").collect();
                    if parts.len() == 2 {
                        if let (Ok(first), Ok(second)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                            rules.push((first, second));
                        }
                    }
                } else {
                    let update_parts: Vec<i32> = line.split(",")
                        .filter_map(|s| s.parse::<i32>().ok())
                        .collect();
                    updates.push(update_parts);
                }
            }

            let correct_updates: Vec<Vec<i32>> = updates
                .clone()
                .into_iter()
                .filter(|u| {
                    let mut correct = true;
                    for i in 0..u.len() {
                        let should_be_after: Vec<i32> = rules
                            .iter()
                            .filter(|&&(a, _)| a == u[i])
                            .map(|&(_, b)| b)
                            .collect();

                        let should_be_before: Vec<i32> = rules
                            .iter()
                            .filter(|&&(_, b)| b == u[i])
                            .map(|&(a, _)| a)
                            .collect();
                        
                        let numbers_before = &u[..i];
                        let numbers_after = &u[i+1..];

                        if numbers_before.iter().any(|&n| should_be_after.contains(&n)) {
                            correct = false;
                            break;
                        }

                        if numbers_after.iter().any(|&n| should_be_before.contains(&n)) {
                            correct = false;
                            break;
                        }
                    }

                    correct
                })
                .collect();
        
            let answer: i32 = correct_updates
                .clone()
                .into_iter()
                .map(|x| {
                    if x.is_empty() {
                        0 
                    } else {
                        x[x.len() / 2]
                    }
                })
                .sum();
            
            println!("Day 5.1: {}", answer);

            let uncorrect_updates: Vec<Vec<i32>> = updates
                .into_iter()
                .filter(|u| !correct_updates.contains(&u))
                .collect();

            let reordered_updates: Vec<Vec<i32>> = uncorrect_updates
                .clone()
                .into_iter()
                .map(|mut u| {
                    let mut changed = true;
                    while changed {
                        changed = false;
                        for i in 0..u.len() {
                            let should_be_after: Vec<i32> = rules
                                .iter()
                                .filter(|&&(a, _)| a == u[i])
                                .map(|&(_, b)| b)
                                .collect();

                            let should_be_before: Vec<i32> = rules
                                .iter()
                                .filter(|&&(_, b)| b == u[i])
                                .map(|&(a, _)| a)
                                .collect();
                            
                            let numbers_before = &u[..i];
                            let numbers_after = u[i+1..].to_vec(); 

                            // swap positions accordingly to rules

                            if let Some(&n) = numbers_before.iter().find(|&&n| should_be_after.contains(&n)) {
                                if let Some(pos) = u.iter().position(|&x| x == n) {
                                    u.swap(i, pos);
                                    changed = true;
                                }
                            }

                            if let Some(&n) = numbers_after.iter().find(|&&n| should_be_before.contains(&n)) {
                                if let Some(pos) = u.iter().position(|&x| x == n) {
                                    u.swap(i, pos);
                                    changed = true;
                                }
                            }
                        }
                    }
                    u
                })
                .collect();
            
            let answer2: i32 = reordered_updates
                .clone()
                .into_iter()
                .map(|x| {
                    if x.is_empty() {
                        0 
                    } else {
                        x[x.len() / 2]
                    }
                })
                .sum();
            
            println!("Day 5.2: {}", answer2);

        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }
}