use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn solve(building: &str) -> i32 {
    let mut sol = 0;
    for c in building.chars() {
        match c {
            '(' => sol += 1,
            ')' => sol -= 1,
            _ => continue,
        }
    }
    sol
}

fn solve_part_2(building: &str) -> Option<usize> {
    let mut sol = 0;
    for (idx, c) in building.chars().enumerate() {
        match c {
            '(' => sol += 1,
            ')' => sol -= 1,
            _ => continue,
        }

        if sol < 0 {
            return Some(idx + 1);
        }
    }

    None
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/Programming/rust/AOC2015/input/day1.txt")?;
    let reader = BufReader::new(file);

    let line = reader.lines().nth(0).unwrap().unwrap();
    println!("{}", line);

    let result = solve(&line);
    println!("result part 1: {}", result);

    if let Some(result) = solve_part_2(&line) {
        println!("result part 2: {}", result);
    }

    Ok(())
}
