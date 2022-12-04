use ferris_says::say;
use std::io::{stdout, BufWriter, Error, Write};
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let stdout = stdout();
    let message = String::from("Craig's Advent of Code Runner 2022");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer)?;
    writer.flush()?;

    println!("Day 1 Part 1, max calories: {}", day1p1(load_input(1)));
    println!( "Day 1 Part 2, total calories from top 3 elves: {}", day1p2(load_input(1)));

    Ok(())
}

fn load_input(day: u8) -> String {
    let path = format!("resources/day{}.txt", day);
    std::fs::read_to_string(path).expect("Unable to read file")
}

fn day1p1(input: String) -> i32 {
    input
        .split("\n\n")
        .map(|batch|
            batch
                .lines()
                .map(|num| num.parse::<i32>().unwrap())
                .sum()
        )
        .max()
        .expect("No batches parsed")
}

fn day1p2(input: String) -> i32 {
    input
        .split("\n\n")
        .map(|batch|
            batch
                .lines()
                .map(|num| num.parse::<i32>().unwrap())
                .sum::<i32>()
        )
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum()
}
