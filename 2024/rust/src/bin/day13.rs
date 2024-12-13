use aoc24::read_day_input;
use regex::Regex;
use std::cmp::max;
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(13);
    let equations: Vec<Equation> = input
        .split("\n\n")
        .map(|e| {
            e.parse()
                .unwrap_or_else(|_| panic!("Failed to parse equation: {e}"))
        })
        .collect();
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let valid = part1(&equations);
    println!(
        "Part 1: '{valid}' took {}ms",
        sw_part1.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(equations: &[Equation]) -> i32 {
    equations
        .iter()
        .map(|e| {
            let mut solutions: Vec<(i32, i32)> = vec![];
            let limit = max(e.x_target, e.y_target);
            for a in 0..limit {
                for b in 0..limit {
                    if a * e.a_x_coeff + b * e.b_x_coeff == e.x_target
                        && a * e.a_y_coeff + b * e.b_y_coeff == e.y_target
                    {
                        solutions.push((a, b));
                    }
                }
            }
            solutions
        })
        .map(|candidates| candidates.iter().map(|(a, b)| 3 * *a + *b).min())
        .flatten()
        .sum()
}

// -------------------- TYPES: EQUATION --------------------

#[derive(Debug)]
struct Equation {
    a_x_coeff: i32,
    a_y_coeff: i32,
    b_x_coeff: i32,
    b_y_coeff: i32,
    x_target: i32,
    y_target: i32,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();
        let fields = pattern.captures(s).unwrap();
        Ok(Self {
            a_x_coeff: fields
                .get(1)
                .map(|f| f.as_str().parse::<i32>().unwrap())
                .unwrap(),
            a_y_coeff: fields
                .get(2)
                .map(|f| f.as_str().parse::<i32>().unwrap())
                .unwrap(),
            b_x_coeff: fields
                .get(3)
                .map(|f| f.as_str().parse::<i32>().unwrap())
                .unwrap(),
            b_y_coeff: fields
                .get(4)
                .map(|f| f.as_str().parse::<i32>().unwrap())
                .unwrap(),
            x_target: fields
                .get(5)
                .map(|f| f.as_str().parse::<i32>().unwrap())
                .unwrap(),
            y_target: fields
                .get(6)
                .map(|f| f.as_str().parse::<i32>().unwrap())
                .unwrap(),
        })
    }
}
