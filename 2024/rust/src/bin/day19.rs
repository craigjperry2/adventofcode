use aoc24::read_day_input;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(19);
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    println!("Parsing took: {}ms", sw_parsing.elapsed().as_millis());

    let sw_part1 = std::time::Instant::now();
    let viable_designs = part1(towels, patterns);
    println!(
        "Part 1: '{viable_designs}' took {}ms",
        sw_part1.elapsed().as_millis(),
    );

    let sw_part2 = std::time::Instant::now();
    let viable_designs = part2(towels, patterns);
    println!(
        "Part 2: '{viable_designs}' took {}ms",
        sw_part2.elapsed().as_millis(),
    );
}

// -------------------- PART 1 --------------------

fn part1(towels: &str, patterns: &str) -> usize {
    let needle: String = [
        "^(".to_string(),
        towels.replace(", ", "|"),
        ")+$".to_string(),
    ]
    .concat();
    let regex = Regex::new(&needle).unwrap();
    patterns.lines().filter(|l| regex.is_match(l)).count()
}

// -------------------- PART 2 --------------------

fn part2(towels: &str, patterns: &str) -> usize {
    let towels: Vec<&str> = towels.split(", ").collect();
    let needle: String = ["^(".to_string(), towels.join("|"), ")+$".to_string()].concat();
    let regex = Regex::new(&needle).unwrap();
    let matches: Vec<String> = patterns
        .lines()
        .filter(|l| regex.is_match(l))
        .map(|m| m.to_string())
        .collect();

    let mut total = 0;
    for pattern in matches {
        let mut dp: Vec<usize> = vec![0; pattern.len() + 1];
        dp[0] = 1; // Empty string is valid

        for i in 0..pattern.len() {
            if dp[i] == 0 {
                continue;
            } // Skip if no valid combinations up to here

            for towel in &towels {
                if pattern[i..].starts_with(towel) {
                    dp[i + towel.len()] += dp[i];
                }
            }
        }

        total += dp[pattern.len()];
    }

    total
}
