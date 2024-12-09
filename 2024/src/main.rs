use std::io;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("Enter the day to run (1-25):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");

    let day: u32 = input.trim().parse().expect("Invalid input. Please enter a number.");

    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        7 => day7::solve(),
        8 => day8::solve(),
        9 => day9::solve(),
        _ => println!("Invalid day. Please enter a number between 1 and 25."),
    }
}