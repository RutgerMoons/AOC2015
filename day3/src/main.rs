use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Point(i32, i32);
#[derive(Clone)]
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

fn visit(start: &Point, instr: &Vec<Instruction>, visited: &mut HashSet<Point>) {
    let mut x: i32 = start.0;
    let mut y: i32 = start.1;
    for ins in instr.iter() {
        match ins {
            Instruction::Up => y += 1,
            Instruction::Down => y -= 1,
            Instruction::Left => x -= 1,
            Instruction::Right => x += 1,
        }

        if !visited.contains(&Point(x, y)) {
            visited.insert(Point(x, y));
        }
    }
}

fn solve(instr: &Vec<Instruction>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point(0, 0));

    visit(&Point(0, 0), instr, &mut visited);

    visited.len()
}

fn solve_part_2(instr: &Vec<Instruction>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point(0, 0));

    let santa_start = Point(0, 0);
    let robot_start = Point(0, 0);

    let santa_instr = instr.iter().enumerate().filter(|(idx, _)| idx % 2 == 0).map(|(_, ins)| ins.clone() ).collect();
    let robot_instr = instr.iter().enumerate().filter(|(idx, _)| idx % 2 == 1).map(|(_, ins)| ins.clone() ).collect();

    visit(&santa_start, &santa_instr, &mut visited);
    visit(&robot_start, &robot_instr, &mut visited);

    visited.len()
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/Programming/rust/AOC2015/input/day3.txt")?;
    let reader = BufReader::new(file);

    let line = reader.lines().nth(0).unwrap().unwrap();
    println!("{}", line);

    let instr: Vec<Instruction> = parse_instructions(&line);

    let result = solve(&instr);
    println!("result part 1: {}", result);

    let result = solve_part_2(&instr);
    println!("result part 2: {}", result);

    Ok(())
}
