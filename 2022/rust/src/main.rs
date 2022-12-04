use ferris_says::say;
use std::{io::{stdout, BufWriter, Error, Write}, collections::{HashMap, HashSet}};
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

    println!("Day 2 Part 1, total score: {}", day2p1(load_input(2)));
    println!("Day 2 Part 2, total score: {}", day2p2(load_input(2)));

    println!("Day 3 Part 1, total score: {}", day3p1(load_input(3)));
    println!("Day 3 Part 2, total score: {}", day3p2(load_input(3)));

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

fn day2p1(input: String) -> i32 {
    let mut lookup = HashMap::new();
    lookup.insert("A X", 4);
    lookup.insert("A Y", 8);
    lookup.insert("A Z", 3);
    lookup.insert("B X", 1);
    lookup.insert("B Y", 5);
    lookup.insert("B Z", 9);
    lookup.insert("C X", 7);
    lookup.insert("C Y", 2);
    lookup.insert("C Z", 6);

    score(&input, &lookup)
}

fn day2p2(input: String) -> i32 {
    let mut lookup = HashMap::new();
    lookup.insert("A X", 3);
    lookup.insert("A Y", 4);
    lookup.insert("A Z", 8);
    lookup.insert("B X", 1);
    lookup.insert("B Y", 5);
    lookup.insert("B Z", 9);
    lookup.insert("C X", 2);
    lookup.insert("C Y", 6);
    lookup.insert("C Z", 7);

    score(&input, &lookup)
}

fn score(input: &String, lookup: &HashMap<&str, i32>) -> i32 {
    input
        .split_terminator("\n")
        .map(|game| {
            lookup.get(&game).expect(format!("No score for {}", game).as_str())
        })
        .sum()             
}

fn priority(item: char) -> i32 {
    (" abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(item))
        .expect("unexpected priority")
        .try_into()
        .expect("priority overflow")
}

fn day3p1(input: String) -> i32 {
    input
        .split_terminator("\n")
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            let left_set: HashSet<char> = left.chars().collect();
            right
                .chars()
                .unique()
                .filter(|c| left_set.contains(c))
                .map(priority)
                .exactly_one()
                .unwrap()
        })
        .sum()
}

fn day3p2(input: String) -> i32 {
    input
        .split_terminator("\n")
        .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .map(|group|
            group
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .expect("No common item in group")
                .into_iter()
                .exactly_one()
                .expect("Multiple common items in group")
        )
        .map(priority)
        .sum()
}
