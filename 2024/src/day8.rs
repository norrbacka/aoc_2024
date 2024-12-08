use std::fs;
use regex::Regex;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use num_cpus;
use rayon::ThreadPoolBuilder; 
use itertools::Itertools; 
use std::collections::{HashSet, HashMap}; 

fn print_map(map: &Vec<Vec<String>>, anti_nodes: &Vec<(i64, i64)>) {
    let mut local_map = map.clone(); 

    for an in anti_nodes {
        if an.1 >= 0 && an.1 < local_map.len() as i64 && 
           an.0 >= 0 && an.0 < local_map[an.1 as usize].len() as i64 
        {
            local_map[an.1 as usize][an.0 as usize] = "#".to_string();
        }
    }

    let flat_map: String = local_map.into_iter() 
        .map(|r| r.join(""))
        .collect::<Vec<String>>() 
        .join("\n"); 
    println!("{}", flat_map);
}

pub fn solve() {
    let file_path = "src/2024_08.txt";
    let mut map: Vec<Vec<String>> = Vec::new();
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            map = contents
                .lines()
                .map(|l| {
                    l.chars().map(|c| c.to_string()).collect::<Vec<String>>() // Collect into Vec<String>
                })
                .collect();
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    }

    let mut char_map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if let Some(c) = ch.chars().next() {
                char_map
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x as i64, y as i64));
            }
        }
    }

    let mut anti_nodes: Vec<(i64, i64)> = Vec::new(); 

    for key in char_map.keys() {
        if *key == '.' { 
            continue;
        }
        let mut pairs = Vec::new();
        for i in 0..char_map[key].len() {
            for j in (i + 1)..char_map[key].len() {
                pairs.push((char_map[key][i], char_map[key][j]));
            }
        }

        for pair in &mut pairs {
            if pair.0 > pair.1 {
                std::mem::swap(&mut pair.0, &mut pair.1);
            }
        }
    
        pairs.sort();

        for pair in pairs {
            let dx = (pair.1.0 - pair.0.0);
            let dy = (pair.1.1 - pair.0.1);
            let c = (pair.0.0 - dx, pair.0.1 - dy);
            let d = (pair.1.0 + dx, pair.1.1 + dy);
            anti_nodes.push(c);
            anti_nodes.push(d);
        }
    }

    anti_nodes.sort();
    anti_nodes.dedup();
    anti_nodes.retain(|&(x, y)| {
        x >= 0 && y >= 0 && y < map.len() as i64 && x < map[y as usize].len() as i64
    });

    //print_map(&map, &anti_nodes);
    println!("Day 8.1: {}", anti_nodes.len());

    anti_nodes.clear();

    for key in char_map.keys() {
        if *key == '.' { 
            continue;
        }
        let mut pairs = Vec::new();
        for i in 0..char_map[key].len() {
            for j in (i + 1)..char_map[key].len() {
                pairs.push((char_map[key][i], char_map[key][j]));
                anti_nodes.push(char_map[key][i]); 
                anti_nodes.push(char_map[key][j]);
            }
        }

        for pair in &mut pairs {
            if pair.0 > pair.1 {
                std::mem::swap(&mut pair.0, &mut pair.1);
            }
        }
    
        pairs.sort();

        for pair in pairs {
            let dx = pair.1.0 - pair.0.0;
            let dy = pair.1.1 - pair.0.1;
            let mut c = (pair.0.0 - dx, pair.0.1 - dy); 
            let mut d = (pair.1.0 + dx, pair.1.1 + dy); 
            
            while c.0 >= 0 && c.1 >= 0 && c.1 < map.len() as i64 && c.0 < map[c.1 as usize].len() as i64 {
                anti_nodes.push(c);
                c = (c.0 - dx, c.1 - dy);
            }
        
            while d.0 >= 0 && d.1 >= 0 && d.1 < map.len() as i64 && d.0 < map[d.1 as usize].len() as i64 {
                anti_nodes.push(d);
                d = (d.0 + dx, d.1 + dy);
            }
        }
    }

    anti_nodes.sort();
    anti_nodes.dedup();
    anti_nodes.retain(|&(x, y)| {
        x >= 0 && y >= 0 && y < map.len() as i64 && x < map[y as usize].len() as i64
    });

    //print_map(&map, &anti_nodes);
    println!("Day 8.2: {}", anti_nodes.len());
}