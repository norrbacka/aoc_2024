use std::io;
mod day1;
mod day2;

fn main() {
    println!("Enter the day to run (1-25):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");

    let day: u32 = input.trim().parse().expect("Invalid input. Please enter a number.");

    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        _ => println!("Invalid day. Please enter a number between 1 and 25."),
    }
}