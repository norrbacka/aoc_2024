use std::fs;
use regex::Regex;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use num_cpus;
use rayon::ThreadPoolBuilder; 
use itertools::Itertools; 
use std::collections::{HashSet, HashMap}; 

pub fn solve() {
    let file_path = "src/2024_09.txt";
    let mut numbers: Vec<i64> = Vec::new();
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            numbers = contents
                .chars() 
                .filter_map(|c| c.to_digit(10)) 
                .map(|d| d as i64) 
                .collect();
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    }

    let mut parsed: Vec<String> = Vec::new();

    let mut id: i64 = 0;
    for i in 0..numbers.len() 
    {
        if i % 2 == 0
        { // file
            for _ in 0..numbers[i]
            {
                parsed.push(id.to_string());
            }            
            id += 1; 
        } 
        else // free space
        {
            for _ in 0..numbers[i]
            {
                parsed.push(".".to_string());
            }
        }
    }

    let mut copy = parsed.clone();
    for i in 0..parsed.len() {
        let j = parsed.len() - i - 1; 
        if copy[j] != "." {
            for k in 0..parsed.len() {
                if copy[k] == "." {
                    copy[k] = copy[j].clone(); 
                    copy[j] = ".".to_string(); 
                    break;
                }
            }
        }
    }
    if copy[0] == "." {
        copy.rotate_left(1);
    }
    
    let mut sum = 0;
    for i in 0..copy.len() {
        match copy[i].parse::<i64>() {
            Ok(num) if copy[i] != "." => {
                sum += num * i as i64;
            },
            _ => {}
        }
    }

    println!("Day 9.1: {}", sum);

    copy = parsed.clone();

    let mut j = copy.len() - 1;
    while j > 0 {
        if copy[j] != "." {
            
            let mut indices: Vec<i64> = Vec::new(); 
            indices.push(j as i64); 
            
            let mut backtrack = j;
            while backtrack > 0 && copy[backtrack - 1] == copy[j] {
                backtrack -= 1;
                indices.push(backtrack as i64);
            }
    
            let size = indices.len(); 

            if size > j {
                break;
            }

            let mut found_space = false;
            let mut target_index = 0;

            for k in 0..copy.len() {
                if (k+size) >= copy.len() {
                    break;
                }
                if (k as i64) >= indices[0] {
                    break;
                }
                let mut found_space_2 = true;
                for z in k..(k+size) {
                    if copy[z] != "." { 
                        found_space_2 = false;
                        break;
                    }
                }
                if found_space_2 {
                    found_space = true;
                    target_index = k;
                    break; 
                }                
            }

            if found_space {
                for (idx, &x) in indices.iter().enumerate() {
                    copy[target_index + idx] = copy[j].clone();
                }
                for &x in &indices { 
                    copy[x as usize] = ".".to_string(); 
                }
                j = j - size;
            } 
            else 
            {
                j = j - size;
            }
        } 
        else 
        {
            j = j - 1;
        }
    }
    
    sum = 0;
    for i in 0..copy.len() {
        match copy[i].parse::<i64>() {
            Ok(num) if copy[i] != "." => {
                sum += num * i as i64;
            },
            _ => {}
        }
    }

    println!("Day 9.2: {}", sum);
}