use aoc24::read_day_input;
use regex::Regex;

fn main() {
    let haystack = read_day_input(3);
    let needle = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut results = vec![];
    for (_, [op1, op2]) in needle.captures_iter(&haystack).map(|c| c.extract()) {
        results.push((
            op1.parse::<i32>().expect("failed to parse op1"),
            op2.parse::<i32>().expect("failed to parse op2"),
        ));
    }

    let result = results.iter().fold(0, |acc, e| (e.0 * e.1) + acc);

    println!("{:?}", result);
}
