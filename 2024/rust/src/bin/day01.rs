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
    let diffs = left.clone().into_iter().zip(right.clone()).map(|(l, r)| (l - r).abs());
    // println!("Differences: {}", diffs.nth(0).unwrap());

    // sum the differences
    let sum: i32 = diffs.sum();
    println!("Part 1: {}", sum);


    // -------------------- PART 2 --------------------

    let mut similarity = 0;

    // iterate left, count number of occurances of left in right
    for l in left {
        let count = right.iter().filter(|&r| r == &l).count();
        similarity += l * count as i32;
    }

    println!("Part 2: {}", similarity);

}
