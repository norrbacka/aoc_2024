use std::fs;
use regex::Regex;
use std::collections::HashSet; 
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use num_cpus;
use rayon::ThreadPoolBuilder; 
use itertools::Itertools; 

pub fn solve() {
    let file_path = "src/2024_07.txt";
    let mut eqs: Vec<(i64, Vec<i64>)> = Vec::new(); 
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            eqs = contents
                .lines()
                .map(|l| {
                    let parts: Vec<&str> = l.split(":").collect();
                    let test_result = parts[0].parse::<i64>().unwrap_or(0); 
                    let numbers = parts[1]
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap_or(0)) 
                        .collect();
                    (test_result, numbers)
                })
                .collect();
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    }

    let mut feqs: Vec<(i64, Vec<i64>)> = Vec::new(); 
 
    let mut operators = vec!["*", "+"];
    for eq in &eqs {
        if eq.1.is_empty() {
            continue;
        }

        let operator_combinations = std::iter::repeat(operators.iter())
            .take(eq.1.len() - 1)
            .multi_cartesian_product();

        for ops in operator_combinations {
            let mut expr = eq.1[0].to_string();
            let mut value = eq.1[0];

            for (i, op) in ops.iter().enumerate() {
                expr.push_str(op);
                expr.push_str(&eq.1[i + 1].to_string());

                value = match **op { 
                    "*" => value * eq.1[i + 1],
                    "+" => value + eq.1[i + 1],
                    _ => value,
                };
            }

            println!("{} = {}", expr, value);

            if value == eq.0 {
                feqs.push(eq.clone());
            }
        }
    }

    let day_1_sum: i64 = 
        feqs
        .iter()
        .collect::<HashSet<_>>() 
        .iter()
        .map(|f| f.0) 
        .sum();

    println!("Day 7.1: {}", day_1_sum);

    feqs.clear();

    operators = vec!["*", "+", "||"];
    for eq in &eqs {
        if eq.1.is_empty() {
            continue;
        }

        let operator_combinations = std::iter::repeat(operators.iter())
            .take(eq.1.len() - 1)
            .multi_cartesian_product();

        for ops in operator_combinations {
            let mut expr = eq.1[0].to_string();
            let mut value = eq.1[0];

            for (i, op) in ops.iter().enumerate() {
                expr.push_str(op);
                expr.push_str(&eq.1[i + 1].to_string());

                value = match **op { 
                    "*" => value * eq.1[i + 1],
                    "+" => value + eq.1[i + 1],
                    "||" => (value.to_string() + &eq.1[i + 1].to_string()).parse::<i64>().unwrap_or(0),                    _ => value,
                };
            }

            println!("{} = {}", expr, value);

            if value == eq.0 {
                feqs.push(eq.clone());
            }
        }
    }

    let day_2_sum: i64 = 
        feqs
        .iter()
        .collect::<HashSet<_>>() 
        .iter()
        .map(|f| f.0) 
        .sum();

    println!("Day 7.2: {}", day_2_sum);
}