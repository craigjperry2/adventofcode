use crate::core::Solution;
use color_eyre::eyre::{Result, bail};

pub struct Day02;

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String> {
        todo!()
    }

    fn part2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

fn parse_rotation(line: &str) -> i32 {
    let n: i32 = line[1..line.len()].parse().unwrap(); // strip leading L/R and trailing '\n'
    if line[0..1] == *"L" { -n } else { n }
}
