use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

struct BoxDim ( u32, u32, u32);

fn solve(dimensions: &Vec<BoxDim>) -> u32 {
    let mut total: u32 = 0;
    for BoxDim(l, w, h) in dimensions {
        let areas: Vec<u32> = vec![l * w, l * h, h * w];
        let min_area = areas.iter().min().unwrap();
        total += min_area + 2 * (areas[0] + areas[1] + areas[2]);
    }
    total
}

fn solve_part_2(dimensions: &Vec<BoxDim>) -> u32 {
    dimensions.iter().map(|BoxDim(l, w, h)| {
        l * w * h + 2 * (l + w + h - l.max(w.max(h)))
    }).fold(0, |acc, x| acc + x)
}

fn line_to_boxdim(line: &str, re_rule: &Regex) -> Option<BoxDim> {
    for cap in re_rule.captures_iter(line) {
        return Some(BoxDim(cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()));
    }
    None
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/Programming/rust/AOC2015/input/day2.txt")?;
    let reader = BufReader::new(file);


    let lines: Vec<String> = reader.lines()
                    .filter_map(|l| l.ok())
                    .collect();

    let re_rule = Regex::new(r"^(\d+)x(\d+)x(\d+)$").unwrap();
    let dimensions: Vec<BoxDim> = lines.iter().map(|line| line_to_boxdim(&line, &re_rule).unwrap()).collect();

    let result = solve(&dimensions);
    println!("result part 1: {}", result);

    let result = solve_part_2(&dimensions);
    println!("result part 2: {}", result);

    Ok(())
}
