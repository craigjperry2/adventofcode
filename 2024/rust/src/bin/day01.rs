use aoc24::read_day_input;
use itertools::Itertools;

fn main() {
    let input = read_day_input(1);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let pairs = input.split_whitespace().chunks(2);
    // let mut first = pairs.into_iter().nth(1).unwrap();
    // let left = first.next().unwrap();
    // let right = first.next().unwrap();
    // println!("Left: {}\tRight: {}", left, right);

    // there must be an idiomatic way to accumulate the left and right of pairs
    // into parallel vectors, using iterators
    for mut pair in pairs.into_iter() {
        left.push(pair.next().unwrap().parse().unwrap());
        right.push(pair.next().unwrap().parse().unwrap());
    }
    // println!("Left: {}\tRight: {}", left[1], right[1]);

    // sort each left and right vector
    left.sort();
    right.sort();

    // calculate absolute difference between left and right
    let diffs = left.into_iter().zip(right).map(|(l, r)| (l - r).abs());
    // println!("Differences: {}", diffs.nth(0).unwrap());

    // sum the differences
    let sum: i32 = diffs.sum();
    println!("Sum: {}", sum);
}
