use std::fs;
use itertools::Itertools; 
use rayon::prelude::*;
use std::collections::HashMap;

pub fn solve() 
{
    let mut numbers = get_numbers();
    for _ in 0..25 {
        blink(&mut numbers);
    }
    println!("Day 11.1: {}", numbers.len());

    let mut numbers2 = get_numbers();
    blink_faster(&mut numbers2, 75);
}

fn get_numbers() -> Vec<i64> {
    let file_path = "src/2024_11.txt";
    match fs::read_to_string(file_path) {
        Ok(contents) => contents
            .split(" ")
            .map(|d| d.parse::<i64>().unwrap())
            .collect(),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Vec::new()
        }
    }
}

fn blink(numbers: &mut Vec<i64>) { 
    let mut new_numbers: Vec<i64> = Vec::new();
    for n in numbers.iter() {
        if *n == 0 { 
            new_numbers.push(1);
            continue;
        }
        
        let string_rep: Vec<char> = n.to_string().chars().collect();
        let even_numbers = string_rep.len() % 2 == 0;
        if even_numbers {
            let mid = string_rep.len() / 2;
            let first_half: String = string_rep[..mid].iter().collect();
            let second_half: String = string_rep[mid..].iter().collect();
            
            new_numbers.push(first_half.parse::<i64>().unwrap());
            new_numbers.push(second_half.parse::<i64>().unwrap());

            continue;
        }

        new_numbers.push(n * 2024);
    }

    *numbers = new_numbers;
}

fn blink_faster(numbers: &mut Vec<i64>, iterations: i64) {
    let mut number_counts: HashMap<i64, i64> = HashMap::new();

    for &number in numbers.iter() {
        *number_counts.entry(number).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        let mut new_counts: HashMap<i64, i64> = HashMap::new();

        for (&number, &count) in number_counts.iter() {
            if number == 0 {
                *new_counts.entry(1).or_insert(0) += count;
            } else {
                let string_rep: Vec<char> = number.to_string().chars().collect();
                let even_numbers = string_rep.len() % 2 == 0;
                if even_numbers {
                    let mid = string_rep.len() / 2;
                    let first_half: i64 = string_rep[..mid].iter().collect::<String>().parse().unwrap();
                    let second_half: i64 = string_rep[mid..].iter().collect::<String>().parse().unwrap();
                    
                    *new_counts.entry(first_half).or_insert(0) += count;
                    *new_counts.entry(second_half).or_insert(0) += count;
                } else {
                    let new_value = number * 2024;
                    *new_counts.entry(new_value).or_insert(0) += count;
                }
            }
        }

        number_counts = new_counts;
    }

    let total_count: i64 = number_counts.values().sum();
    println!("Day 11.2: {}", total_count);
}