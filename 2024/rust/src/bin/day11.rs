use aoc24::read_day_input;
use std::collections::HashMap;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(11);
    let stones: Vec<String> = input.split_whitespace().map(|s| s.to_owned()).collect();
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let valid = part1(&stones);
    println!(
        "Part 1: '{valid}' took {}ms",
        sw_part1.elapsed().as_millis()
    );

    let sw_part2 = std::time::Instant::now();
    let valid = part2(&stones);
    println!(
        "Part 2: '{valid}' took {}ms",
        sw_part2.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(start_stones: &Vec<String>) -> usize {
    let mut stones: Vec<String> = start_stones.clone();
    for _ in 0..25 {
        stones = stones
            .iter()
            .flat_map(|s| {
                if s == "0" {
                    vec![String::from("1")]
                } else if s.len() % 2 == 0 {
                    let half = s.len() / 2;
                    vec![
                        s[..half].to_owned(),
                        s[half..].to_string().parse::<i32>().unwrap().to_string(),
                    ]
                } else {
                    vec![(s.parse::<i64>().unwrap() * 2024).to_string()]
                }
            })
            .collect();
    }
    stones.len()
}

// -------------------- PART 2 --------------------

// REJECTED: was only marginally faster than part1
// fn part2_inverted_loop_ordering(start_stones: &Vec<String>) -> usize {
//     let mut end_stones: Vec<String> = Vec::new();
//     for stone in start_stones {
//         let mut result = vec![stone.clone()];
//         for i in 0..25 {
//             // println!("{i}");
//             result = result.iter().map(|r| process(r)).flatten().collect();
//         }
//         end_stones.extend(result);
//         // println!("end: {end_stones:?}");
//     }
//     end_stones.len()
// }
//
// fn process(s: &str) -> Vec<String> {
//     let result = if s == "0" {
//         vec![String::from("1")]
//     } else if s.len() % 2 == 0 {
//         let half = s.len() / 2;
//         vec![
//             s[..half].to_owned(),
//             s[half..].to_string().parse::<i32>().unwrap().to_string(),
//         ]
//     } else {
//         vec![(s.parse::<i64>().unwrap() * 2024).to_string()]
//     };
//     result
// }

// REJECTED: i thought i saw a relationship to fibonacci when i plot the 7 known test cases in excel, 5 lined up, but i'm wrong here...
// fn part2_fib() -> usize {
//     // after a bunch of pattern matching in excel i see some kind of fibonacci relationship but...
//     let mut answer: usize = 0;
//     for blink in 1..=75 {
//         let even = blink % 2 == 0;
//         // let step = if even { fibonacci(blink - 1) + 1 } else { fibonacci(blink - 1) - 1 };
//         answer = if even { fibonacci(blink + 1 ) + 1 } else { fibonacci(blink + 1)  };
//         // sum += step;
//         println!("blink:{} sum:{} fib:{}", blink, answer, fibonacci(blink));
//         // println!("blink:{} sum:{} step:{} diff:{} fib:{}", blink, sum, step, sum - step, fibonacci(blink));
//     }
//     answer // only gives the correct answer for 5 out of 7 test cases
// }

// wrong 1 -> 2 should be 3
// 2 -> 4
// 3 -> 5
// 4 -> 9
// 5 -> 13
// 6 -> 22
// wrong 25 -> 196418 should be 194782

// fn fibonacci(n: usize) -> usize {
//     let mut a = 0;
//     let mut b = 1;
//     let mut c = 0;
//     for _ in 0..n {
//         c = a + b;
//         a = b;
//         b = c;
//     }
//     c
// }

fn part2(start_stones: &Vec<String>) -> usize {
    // don't use a list, use a counter...
    let mut buckets: HashMap<usize, usize> = start_stones
        .iter()
        .map(|s| (s.parse().unwrap(), 1))
        .collect(); // NB: assumes starting list is unique

    for _ in 0..75 {
        buckets = buckets
            .iter()
            .flat_map(|(&stone, &count)| {
                if stone == 0 {
                    vec![(1, count)]
                } else if stone.to_string().len() % 2 == 0 {
                    let half = stone.to_string().len() / 2;
                    vec![
                        (stone.to_string()[..half].parse().unwrap(), count),
                        (stone.to_string()[half..].parse().unwrap(), count),
                    ]
                } else {
                    vec![(stone * 2024, count)]
                }
            })
            .fold(HashMap::new(), |mut acc, (stone, count)| {
                *acc.entry(stone).or_insert(0) += count;
                acc
            });
    }

    buckets.values().sum()
}
