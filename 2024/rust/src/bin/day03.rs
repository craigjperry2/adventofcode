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

    println!("Part 1: {:?}", result);

    // -------------------- PART 2 --------------------

    let instructions = Regex::new(r"(don\'t)|(do)|(mul\(\d+,\d+\))").unwrap();

    let mut results: Vec<Instruction> = vec![];
    for (_, [cmd]) in instructions.captures_iter(&haystack).map(|c| c.extract()) {
        // TODO: this could be moved to a FromStr trait impl i believe
        if cmd == "don't" {
            results.push(Instruction::Dont);
        } else if cmd == "do" {
            results.push(Instruction::Do);
        } else {
            let (_, [op1, op2]) = needle
                .captures(cmd)
                .and_then(|c| Some(c.extract()))
                .expect("Failed to parse ops");

            results.push(Instruction::Mul(
                op1.parse::<i32>().expect("failed to parse op1"),
                op2.parse::<i32>().expect("failed to parse op2"),
            ));
        }
    }

    let mut enabled = true;
    let mut result = 0;
    for e in results {
        match e {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(op1, op2) => {
                if enabled {
                    result += op1 * op2
                }
            }
        }
    }

    println!("Part 2: {:?}", result);
}

enum Instruction {
    Dont,
    Do,
    Mul(i32, i32),
}
