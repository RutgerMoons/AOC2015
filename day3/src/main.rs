use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Point(i32, i32);
enum Instruction {
    Up, Down, Left, Right
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instr: Vec<Instruction> = Vec::new();

    for c in input.chars() {
        match c {
            '^' => instr.push(Instruction::Up),
            'v' => instr.push(Instruction::Down),
            '<' => instr.push(Instruction::Left),
            '>' => instr.push(Instruction::Right),
            _ => continue,
        }
    }
    
    instr
}

fn solve(instr: &Vec<Instruction>) -> i64 {
    let mut nb_houses: i64 = 1;
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point(0, 0));

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for ins in instr.iter() {
        match ins {
            Instruction::Up => y += 1,
            Instruction::Down => y -= 1,
            Instruction::Left => x -= 1,
            Instruction::Right => x += 1,
        }

        if !visited.contains(&Point(x, y)) {
            nb_houses += 1;
            visited.insert(Point(x, y));
        }
    }

    nb_houses
}

fn solve_part_2(building: &str) -> Option<usize> {
    unimplemented!()
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/Programming/rust/AOC2015/input/day3.txt")?;
    let reader = BufReader::new(file);

    let line = reader.lines().nth(0).unwrap().unwrap();
    println!("{}", line);

    let instr: Vec<Instruction> = parse_instructions(&line);

    let result = solve(&instr);
    println!("result part 1: {}", result);

    if let Some(result) = solve_part_2(&line) {
        println!("result part 2: {}", result);
    }

    Ok(())
}
