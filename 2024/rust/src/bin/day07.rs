use aoc24::read_day_input;
use std::collections::VecDeque;
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(7);
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            line.parse()
                .unwrap_or_else(|()| panic!("Failed to parse equation: {line}"))
        })
        .collect();
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let valid = part1(&equations);
    println!(
        "Part 1: '{valid}' took {}ms",
        sw_part1.elapsed().as_millis()
    );

    let sw_part2 = std::time::Instant::now();
    let valid = part2(&equations);
    println!(
        "Part 2: '{valid}' took {}ms",
        sw_part2.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .filter(|e| crate::has_solution(e.result, 0, &mut e.terms.clone()))
        .map(|e| e.result)
        .sum()
}

fn has_solution(target: i64, total: i64, terms: &mut VecDeque<i64>) -> bool {
    if terms.is_empty() {
        return total == target;
    }

    let term = terms.pop_front().unwrap();

    has_solution(target, total + term, &mut terms.clone())
        || has_solution(target, total * term, terms)
}

// -------------------- PART 2 --------------------

fn part2(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .filter(|e| has_concat_solution(e.result, 0, &mut e.terms.clone()))
        .map(|e| e.result)
        .sum()
}

fn has_concat_solution(target: i64, total: i64, terms: &mut VecDeque<i64>) -> bool {
    if terms.is_empty() {
        return total == target;
    }

    let term = terms.pop_front().unwrap();

    has_concat_solution(target, total + term, &mut terms.clone())
        || has_concat_solution(target, total * term, &mut terms.clone())
        || has_concat_solution(
            target,
            (total.to_string() + &term.to_string()).parse().unwrap(),
            &mut terms.clone(),
        )
}

// -------------------- TYPES: EQUATION --------------------

struct Equation {
    result: i64,
    terms: VecDeque<i64>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_result, raw_terms) = s
            .split_once(": ")
            .unwrap_or_else(|| panic!("Failed to split equation: {s}"));

        let result: i64 = raw_result
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse result: {raw_result}"));

        let terms: VecDeque<i64> = raw_terms
            .split(' ')
            .map(|term| {
                term.parse()
                    .unwrap_or_else(|_| panic!("Failed to parse term: {term}"))
            })
            .collect();

        Ok(Self { result, terms })
    }
}
