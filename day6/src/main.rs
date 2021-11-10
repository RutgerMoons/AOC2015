use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

#[derive(Debug)]
struct Point(usize, usize);

#[derive(Debug)]
struct Range {
    start: Point,
    end: Point,
}

#[derive(Debug)]
enum Instruction {
    TurnOn(Range),
    Toggle(Range),
    TurnOff(Range),
}

type Grid<T> = Vec<Vec<T>>;

fn str_to_instr(s: &str) -> Option<Instruction> {
    let re_instr = Regex::new(r"^(turn on|turn off|toggle)\W*(\d+),(\d+) through (\d+),(\d+)$").unwrap();
    for cap in re_instr.captures_iter(s) {
        let start = Point(cap[2].parse().unwrap(), cap[3].parse().unwrap());
        let end   = Point(cap[4].parse().unwrap(), cap[5].parse().unwrap());
        let range = Range { start: start, end: end };
        let instr = match &cap[1] {
            "turn on" => Some(Instruction::TurnOn(range)),
            "turn off" => Some(Instruction::TurnOff(range)),
            "toggle" => Some(Instruction::Toggle(range)),
            _ => None,
        };
        return instr;
    }

    None
}

fn light_grid(grid: &mut Grid<bool>, instructions: &Vec<Instruction>) {
    for instr in instructions {
        match instr {
            Instruction::TurnOn(r) => {
                for x in r.start.0 ..= r.end.0 {
                    for y in r.start.1 ..= r.end.1 {
                        grid[x][y] = true;
                    }
                }
            },
            Instruction::TurnOff(r) => {
                for x in r.start.0 ..= r.end.0 {
                    for y in r.start.1 ..= r.end.1 {
                        grid[x][y] = false;
                    }
                }
            },
            Instruction::Toggle(r) => {
                for x in r.start.0 ..= r.end.0 {
                    for y in r.start.1 ..= r.end.1 {
                        grid[x][y] = !grid[x][y];
                    }
                }
            }
        }
    }
}

fn light_grid_u32(grid: &mut Grid<u32>, instructions: &Vec<Instruction>) {
    for instr in instructions {
        match instr {
            Instruction::TurnOn(r) => {
                for x in r.start.0 ..= r.end.0 {
                    for y in r.start.1 ..= r.end.1 {
                        grid[x][y] += 1;
                    }
                }
            },
            Instruction::TurnOff(r) => {
                for x in r.start.0 ..= r.end.0 {
                    for y in r.start.1 ..= r.end.1 {
                        if grid[x][y] == 0 {
                            continue;
                        } else {
                            grid[x][y] -=  1;
                        }
                    }
                }
            },
            Instruction::Toggle(r) => {
                for x in r.start.0 ..= r.end.0 {
                    for y in r.start.1 ..= r.end.1 {
                        grid[x][y] += 2;
                    }
                }
            }
        }
    }
}

fn amount_lit(grid: &Grid<bool>) -> u32 {
    grid.iter()
        .map(|row| row.iter().fold(0, |acc, light| if *light {acc + 1} else { acc })) // row sum
        .fold(0, |acc, row_count| acc + row_count) // sum of rows

}

fn amount_lit_u32(grid: &Grid<u32>) -> u32 {
    grid.iter()
        .map(|row| row.iter().fold(0, |acc, light| acc + light )) // row sum
        .fold(0, |acc, row_count| acc + row_count) // sum of rows
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/Programming/rust/AOC2015/input/day6.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .filter_map(|l| l.ok())
                    .collect();

    let instructions: Vec<Instruction> = lines.iter().map(|s| str_to_instr(s).unwrap()).collect();
    let mut grid: Grid<bool> = vec!(vec!(false; 1000); 1000);
    light_grid(&mut grid, &instructions);
    let result  = amount_lit(&grid);
    println!("result part 1: {}", result);

    let mut grid: Grid<u32> = vec!(vec!(0; 1000); 1000);
    light_grid_u32(&mut grid, &instructions);
    let result  = amount_lit_u32(&grid);
    println!("result part 2: {}", result);

    Ok(())
}
