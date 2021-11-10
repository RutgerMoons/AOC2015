use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn is_nice(input: &str) -> bool {
    let re_vowel = Regex::new(r"a|e|i|o|u").unwrap();
    let ctr = re_vowel.captures_iter(input).count();
    if ctr < 3 {
        return false;
    }

    let re_double = Regex::new(r"aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz").unwrap();
    if !re_double.is_match(input) {
        return false;
    }

    let re_bad = Regex::new(r"ab|cd|pq|xy").unwrap();
    !re_bad.is_match(input)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/Programming/rust/AOC2015/input/day5.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .filter_map(|l| l.ok())
                    .collect();

    /*
    let s = "aeiouae";
    println!("{} {}", s, is_nice(s));
    */

    let result = lines.iter().map(|s| is_nice(s)).fold(0, |acc, x| if x { acc + 1 } else { acc });
    println!("result part 1: {}", result);

    /*
    let result = solve(&dimensions);

    let result = solve_part_2(&dimensions);
    println!("result part 2: {}", result);
    */

    Ok(())
}
