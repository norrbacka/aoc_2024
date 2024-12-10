use std::fs;
use itertools::Itertools; 

pub fn solve() {
    let file_path = "src/2024_10.txt";
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

    let mut entry_points: Vec<(usize, usize)> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == "0" {
                entry_points.push((y as usize, x as usize));
            }
        }
    }

    let mut hike_sum: i32 = 0;

    for entry_point in &entry_points {
        let mut path: Vec<(usize, usize)> = Vec::new();

        traverse(&map, &entry_point, &mut path);

        path.sort_unstable();
        path.dedup();
    
        //print_map_with_path(&map, &path);
    
        for p in &path {
            if map[p.0][p.1] == "9" {
                hike_sum = hike_sum + 1;
            }
        }
    } 

    println!("Day 10.1: {}", hike_sum);

    let mut hike_sum: i32 = 0;

    for entry_point in &entry_points {
        let mut path: Vec<(usize, usize)> = Vec::new();

        traverse(&map, &entry_point, &mut path);

        for p in &path {
            if map[p.0][p.1] == "9" {
                hike_sum = hike_sum + 1;
            }
        }
    } 

    println!("Day 10.2: {}", hike_sum);
}

pub fn traverse(
    map: &Vec<Vec<String>>, 
    point: &(usize, usize), 
    path: &mut Vec<(usize, usize)>, 
) {
    path.push(*point); 
    let y = point.0;
    let x = point.1;

    let current_value = &map[y][x];
    //println!("Current value at ({}, {}): {}", y, x, current_value);

    let next_level = match current_value.parse::<usize>() {
        Ok(val) => val + 1,
        Err(e) => {
            eprintln!("Error parsing value at ({}, {}): {}", y, x, e);
            return;
        }
    };
    
    let max_y = map.len();
    let max_x = map[0].len();

    let can_go_up = y > 0;
    if can_go_up {
        let up: (usize, usize) = (y - 1, x);
        if let Ok(value) = map[up.0][up.1].parse::<usize>() {
            if value == next_level {
                traverse(&map, &up, path);
            }
        }
    }

    let can_go_down = y < max_y - 1;
    if can_go_down {
        let down: (usize, usize) = (y + 1, x);
        if let Ok(value) = map[down.0][down.1].parse::<usize>() {
            if value == next_level {
                traverse(&map, &down, path);
            }
        }
    }
    
    let can_go_left = x > 0;
    if can_go_left {
        let left: (usize, usize) = (y, x - 1);
        if let Ok(value) = map[left.0][left.1].parse::<usize>() {
            if value == next_level {
                traverse(&map, &left, path);
            }
        }
    }
    
    let can_go_right = x < max_x - 1;
    if can_go_right {
        let right: (usize, usize) = (y, x + 1);
        if let Ok(value) = map[right.0][right.1].parse::<usize>() {
            if value == next_level {
                traverse(&map, &right, path);
            }
        }
    }
}

fn print_map_with_path(map: &Vec<Vec<String>>, path: &Vec<(usize, usize)>) {
    let mut map_copy = map.clone();
    for p in path {
        map_copy[p.0][p.1] = "X".to_string();
    }
    println!("{}", map_copy.iter().map(|x| x.join("")).join("\n"));
}
