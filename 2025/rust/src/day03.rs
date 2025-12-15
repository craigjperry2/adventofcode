use crate::core::Solution;
use crate::parsers::{digits, lines, parse_day_with_preview};
use color_eyre::eyre::Result;

pub struct Day03;

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String> {
        let banks = parse_day_with_preview(3, |x| x.to_string(), lines, 5)?;
        Ok(total_joltage(&banks).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

pub fn joltage(bank: &str) -> String {
    let bytes = bank.as_bytes();
    assert!(bytes.len() >= 2, "bank must have at least 2 digits");

    let mut first = bytes[0];
    for &b in &bytes[..bytes.len() - 1] {
        if b > first {
            first = b;
        }
    }

    let first_pos = bytes
        .iter()
        .position(|&b| b == first)
        .expect("first must exist in bank");
    assert!(first_pos + 1 < bytes.len(), "no digit after first");

    let mut second = bytes[first_pos + 1];
    for &b in &bytes[first_pos + 1..] {
        if b > second {
            second = b;
        }
    }

    String::from_utf8(vec![first, second]).expect("digits should be valid UTF-8")
}

pub fn total_joltage(banks: &[String]) -> i32 {
    banks
        .iter()
        .map(|bank| joltage(bank).parse::<i32>().expect("two digits should parse"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joltage_smoke_test() {
        assert_eq!(joltage("8647"), "87");
        assert_eq!(joltage("1119"), "19");
    }
}
