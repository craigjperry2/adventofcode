use aoc24::read_day_input;
use regex::Regex;
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
    println!("Parsing took: {}ms", sw_parsing.elapsed().as_millis());

    let sw_part1 = std::time::Instant::now();
    let valid = part1(&equations);
    println!("Part 1: '{valid}' took {}µs", sw_part1.elapsed().as_micros(),);

    let sw_part2 = std::time::Instant::now();
    let valid = part2(&equations);
    println!(
        "Part 2: '{valid}' took {}µs",
        sw_part2.elapsed().as_micros()
    );
}

// -------------------- PART 1 --------------------

fn part1(equations: &[Equation]) -> i32 {
    equations
        .iter()
        .map(|e| {
            let mut solutions: Vec<(i32, i32)> = vec![];
            let limit = 100;
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
        .filter_map(|candidates| candidates.iter().map(|(a, b)| 3 * *a + *b).min())
        .sum()
}

// -------------------- PART 2 --------------------

fn part2(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .map(|e| Equation2 {
            a_x_coeff: e.a_x_coeff as f64,
            a_y_coeff: e.a_y_coeff as f64,
            b_x_coeff: e.b_x_coeff as f64,
            b_y_coeff: e.b_y_coeff as f64,
            x_target: e.x_target as f64 + 10000000000000.0,
            y_target: e.y_target as f64 + 10000000000000.0,
        })
        .filter_map(|e| {
            // Interpreting the input as a pair of equations:
            //   x_target = a_x * a + b_x * b
            //   y_target = a_y * a + b_y * b

            // then:
            //   a = rounded( (y_target / b_y - x_target / b_x) / (a_y / b_y - a_x / b_x) )
            let a = ((e.y_target / e.b_y_coeff - e.x_target / e.b_x_coeff)
                / (e.a_y_coeff / e.b_y_coeff - e.a_x_coeff / e.b_x_coeff))
                .round();

            // and:
            //   b = rounded( (x_target - a * a_x) / b_x  )
            let b = ((e.x_target - a * e.a_x_coeff) / e.b_x_coeff).round();

            // Since we only care about whole number solutions, make sure the rounded answers work:
            //   valid = ( a * a_x + b * bx == x_target ) AND ( y_target == ... )
            if a * e.a_x_coeff + b * e.b_x_coeff == e.x_target
                && a * e.a_y_coeff + b * e.b_y_coeff == e.y_target
            {
                #[allow(clippy::cast_possible_truncation)]
                Some((3.0 * a + b).round() as i64)
            } else {
                None
            }
        })
        // .inspect(|&cost| println!("{cost}"))
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

#[derive(Debug)]
struct Equation2 {
    a_x_coeff: f64,
    a_y_coeff: f64,
    b_x_coeff: f64,
    b_y_coeff: f64,
    x_target: f64,
    y_target: f64,
}
