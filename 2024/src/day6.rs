use std::fs;
use regex::Regex;
use std::collections::HashSet; 
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use num_cpus;
use rayon::ThreadPoolBuilder; 

pub fn solve() {

    let file_path = "src/2024_06.txt";
    let mut orig_map: Vec<Vec<String>> = Vec::new(); 
    let mut map: Vec<Vec<String>> = Vec::new(); 
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            orig_map = contents
                .lines()
                .map(|l| {
                    l.chars().map(|c| c.to_string()).collect()
                })
                .collect(); 
            map = orig_map.clone();
        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }

    let mut guard_is_in_sight: bool = true;
     while(guard_is_in_sight) 
    {
        let flat_map: String = map.clone()
            .into_iter() 
            .map(|r| r.join(""))
            .collect::<Vec<String>>() 
            .join("\n"); 

        guard_is_in_sight = 
            flat_map.contains("^") ||
            flat_map.contains(">") ||
            flat_map.contains("<") ||
            flat_map.contains("v");
        
        if(!guard_is_in_sight) 
        {
            let count = map.clone()
                .into_iter()
                .flat_map(|x| x.into_iter()) 
                .filter(|x| x == "X")
                .count();
            println!("Day 6.1: {}", count);
            break;
        }

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                match map[y][x].as_str() {
                    "^" => {
                        map[y][x] = "X".to_string();
                        if y > 0 && map[y - 1][x] != "#" {
                            map[y - 1][x] = "^".to_string();
                        } else if y > 0 {
                            map[y][x] = ">".to_string();
                        }
                    }
                    ">" => {
                        map[y][x] = "X".to_string();
                        if x + 1 < map[y].len() && map[y][x + 1] != "#" {
                            map[y][x + 1] = ">".to_string();
                        } else if x + 1 < map[y].len() {
                            map[y][x] = "v".to_string();
                        }
                    }
                    "<" => {
                        map[y][x] = "X".to_string();
                        if x > 0 && map[y][x - 1] != "#" {
                            map[y][x - 1] = "<".to_string();
                        } else if x > 0 {
                            map[y][x] = "^".to_string();
                        }
                    }
                    "v" => {
                        map[y][x] = "X".to_string();
                        if y + 1 < map.len() && map[y + 1][x] != "#" {
                            map[y + 1][x] = "v".to_string();
                        } else if y + 1 < map.len() {
                            map[y][x] = "<".to_string();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let paths_walked: Vec<(usize, usize)> =
    map
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(x, value)| {
                if value == "X" {
                    Some((x, y))
                } else {
                    None
                }
            })
    })
    .collect::<HashSet<_>>()
    .into_iter()
    .collect(); 

    let options: Vec<(usize, usize)> = orig_map
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, value)| if value == "." { Some((x, y)) } else { None })
        })
        .collect();

    
    let mut orig_x = 0;
    let mut orig_y = 0;
    for (dy, row) in orig_map.iter().enumerate() {
        if let Some(dx) = row.iter().position(|cell| cell == "^") {
            orig_x = dx;
            orig_y = dy;
            break;
        }
    }

    let loops = AtomicUsize::new(0);
    let i = AtomicUsize::new(0);
    let total_options = paths_walked.len();

    let max_threads = num_cpus::get();

    let pool = ThreadPoolBuilder::new()
        .num_threads(max_threads)
        .build()
        .unwrap();


    pool.install(|| {
        paths_walked.par_iter().for_each(|&obstruction| {
            i.fetch_add(1, Ordering::SeqCst);
            let current_i = i.load(Ordering::SeqCst);
            println!("Checking {}/{} = {:.2}%", current_i, total_options, (current_i as f64 / total_options as f64) * 100.0);
            
            let mut walked = std::collections::HashSet::new();
            let mut map = orig_map.clone();
            let mut guard_is_in_sight: bool = true;
        
            map[obstruction.1][obstruction.0] = "#".to_string();
    
            
            let mut x = orig_x.clone();
            let mut y = orig_y.clone();
        
            while guard_is_in_sight {
    
                guard_is_in_sight = map.par_iter().any(|row| {
                    row.iter().any(|cell| cell == "^" || cell == ">" || cell == "<" || cell == "v")
                });
    
                if !guard_is_in_sight {
                    println!("Noting on: ({},{})", obstruction.1, obstruction.0);
                    break;
                }
    
                match map[y][x].as_str() {
                    "^" => {
                        map[y][x] = "X".to_string();
                        if y > 0 && map[y - 1][x] != "#" {
                            if map[y - 1][x] == "X" {
                                if !walked.insert((y, x, "^".to_string())) {
                                    loops.fetch_add(1, Ordering::SeqCst);
                                    println!("Found: {} \n", loops.load(Ordering::SeqCst));
                                    return; 
                                }
                            }
                            map[y - 1][x] = "^".to_string();
                            y = y - 1;
                        } else if y > 0 {
                            map[y][x] = ">".to_string();
                        }
                    }
                    ">" => {
                        map[y][x] = "X".to_string();
    
                        if x + 1 < map[y].len() && map[y][x + 1] != "#" {
                            if map[y][x + 1] == "X" {
                                if !walked.insert((y, x, ">".to_string())) {
                                    loops.fetch_add(1, Ordering::SeqCst);
                                    println!("Found: {} \n", loops.load(Ordering::SeqCst));
                                    return; 
                                }
                            }
                            map[y][x + 1] = ">".to_string();
                            x = x + 1;
                        } else if x + 1 < map[y].len() {
                            map[y][x] = "v".to_string();
                        }
                    }
                    "<" => {
                        map[y][x] = "X".to_string();
    
                        if x > 0 && map[y][x - 1] != "#" {
                            if map[y][x - 1] == "X" {
                                if !walked.insert((y, x, "<".to_string())) {
                                    loops.fetch_add(1, Ordering::SeqCst);
                                    println!("Found: {} \n", loops.load(Ordering::SeqCst));
                                    return; 
                                }
                            }
                            map[y][x - 1] = "<".to_string();
                            x = x - 1;
                        } else if x > 0 {
                            map[y][x] = "^".to_string();
                        }
                    }
                    "v" => {
                        map[y][x] = "X".to_string();
    
                        if y + 1 < map.len() && map[y + 1][x] != "#" {
                            if map[y + 1][x] == "X" {
                                if !walked.insert((y, x, "V".to_string())) {
                                    loops.fetch_add(1, Ordering::SeqCst);
                                    println!("Found: {} \n", loops.load(Ordering::SeqCst));
                                    return; 
                                }
                            }
                            map[y + 1][x] = "v".to_string();
                            y = y + 1;
                        } else if y + 1 < map.len() 
                        {
                            map[y][x] = "<".to_string();
                        }
                    }
                    _ => {}
                }
            }
        });
    });

    println!("Day 6.2: {}", loops.load(Ordering::SeqCst));
}