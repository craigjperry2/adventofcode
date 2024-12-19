use aoc24::read_day_input;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(19);
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    println!("Parsing took: {}ms", sw_parsing.elapsed().as_millis());

    let sw_part1 = std::time::Instant::now();
    let viable_designs = part1(towels, patterns);
    println!("Part 1: '{viable_designs}' took {}µs", sw_part1.elapsed().as_micros(), );
}

// -------------------- PART 1 --------------------

fn part1(towels: &str, patterns: &str) -> usize {
    let needle: String = ["^(".to_string(), towels.to_string().split(", ").join("|"), ")+$".to_string()].concat();
    println!("{needle}");
    let regex = Regex::new(&needle).unwrap();
    patterns.lines().filter(|l| regex.is_match(l)).count()
}

