use ferris_says::say;
use std::{io::{stdout, BufWriter, Error, Write}, collections::{HashMap, HashSet}};
use itertools::Itertools;
use regex::Regex;

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

    println!("Day 4 Part 1, total pairs: {}", day4p1(load_input(4)));
    println!("Day 4 Part 2, total pairs: {}", day4p2(load_input(4)));

    println!("Day 5 Part 1, top crates: {}", day5p1(load_input(5)));
    println!("Day 5 Part 2, top crates: {}", day5p2(load_input(5)));

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

fn split_and_parse<T>(input: &str, separator: &str) -> (T, T)
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug
{
    let comma = input.find(separator).expect(format!("No {} in input", separator).as_str());
    let left = &input[..comma];
    let right = &input[comma+1..];

    let left = left.parse::<T>().expect(format!("Left is not a {}", std::any::type_name::<T>()).as_str());
    let right = right.parse::<T>().expect(format!("Right is not a {}", std::any::type_name::<T>()).as_str());

    (left, right)
}

fn day4(input: String) -> Vec<(HashSet<i32>, HashSet<i32>)> {
    input
        .split_terminator("\n")
        .map(|assignments| {
            let (left, right) = split_and_parse::<String>(&assignments, ",");

            let (left_from, left_to) = split_and_parse::<i32>(&left, "-");
            let left_set: HashSet<i32> = (left_from..=left_to).collect();

            let (right_from, right_to) = split_and_parse::<i32>(&right, "-");
            let right_set: HashSet<i32> = (right_from..=right_to).collect();
            (left_set, right_set)
        })
        .collect::<Vec<(HashSet<i32>, HashSet<i32>)>>()
}

fn day4p1(input: String) -> i32 {
    day4(input)
        .iter()
        .map(|(l, r)| (l.is_superset(&r) || r.is_superset(&l)) as i32) 
        .sum()
}

fn day4p2(input: String) -> i32 {
    day4(input)
        .iter()
        .map(|(l, r)| l.intersection(&r).count() + r.intersection(&l).count() > 0)
        .map(|x| x as i32)
        .sum()
}


fn day5(input: String, part1: bool) -> String {
    let (starting_state, instructions) = input.split_terminator("\n\n").collect_tuple().expect("Failed to parse state and instructions");

    let mut state = starting_state
        .lines()
        // discard the last row which contains the column numbers
        .take(8)
        // normalise column spacing to allow striding over every 4th char starting from the 0th position
        .map(|row|
            row.chars()
                .skip(1)
                .collect::<String>()
        )
        // stride over every 4th char to parse the given state into a 2d array
        .map(|row|
            row.chars()
                .enumerate()
                .filter(|(i, _)| i % 4 == 0)
                .map(|(_, c)| c)
                .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    // transpose the state matrix
    state = (0..state[0].len())
        .map(|i|
            state.iter()
                .map(|row| row[i])
                .filter(|c| *c != ' ')
                .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    // parse and execute each instruction
    instructions
        .lines()
        .map(|instruction| {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Failed to compile regex");
            let captures = re.captures(instruction).expect("Failed to parse instruction");
            // parse into a 3-tuple of (count, from, to)
            return (captures[1].parse::<i32>().expect("Failed to parse count"), captures[2].parse::<i32>().expect("Failed to parse from"), captures[3].parse::<i32>().expect("Failed to parse to"));
        })
        .for_each(|(count, from, to)| {
            // NB: 0 indexed vector but 1 indexed instructions, hence the -1
            let mut x = state[(from - 1) as usize].drain(..count as usize).collect::<Vec<char>>();
            if part1 {
                // part 1 requires us to place the items in reverse order just as if we moved 1 box at a time
                x.reverse();
            }
            state[(to - 1) as usize].splice(0..0, x);
        });

    // The top crate of each stack is the final answer
    state
        .iter()
        .map(|row| row[0])
        .collect::<String>()

}

fn day5p1(input: String) -> String {
    day5(input, true)
}

fn day5p2(input: String) -> String {
    day5(input, false)
}
