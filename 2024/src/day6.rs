use std::fs;
use regex::Regex;

pub fn solve() {

    let file_path = "src/2024_06.txt";
    let mut map: Vec<Vec<String>> = Vec::new(); 
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            map = contents
                .lines()
                .map(|l| {
                    l.chars().map(|c| c.to_string()).collect()
                })
                .collect(); 
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

        //println!("\n");
        //println!("{}", &flat_map);

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

}