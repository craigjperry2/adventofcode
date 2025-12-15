use crate::core::Solution;
use crate::parsers::{lines, parse_day_with_preview};
use color_eyre::eyre::Result;

pub struct Day03;

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String> {
        let banks = parse_day_with_preview(3, |x| x.to_string(), lines, 5)?;
        Ok(total_joltage(&banks, 2).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let banks = parse_day_with_preview(3, |x| x.to_string(), lines, 5)?;
        Ok(total_joltage(&banks, 12).to_string())
    }
}

pub fn joltage(bank: &str, n: usize) -> String {
    let bytes = bank.as_bytes();
    assert!(n >= 1, "n must be >= 1");
    assert!(bytes.len() >= n, "bank must have at least n digits");

    let mut out: Vec<u8> = Vec::with_capacity(n);
    let mut start = 0usize;
    let mut remaining = n;

    while remaining > 0 {
        // The next chosen digit can't be so late that we can't pick the rest.
        let end_exclusive = bytes.len() - (remaining - 1);

        // Max digit in the allowed window.
        let mut max_b = bytes[start];
        for &b in &bytes[start..end_exclusive] {
            if b > max_b {
                max_b = b;
            }
        }

        // First occurrence of that max digit within the allowed window.
        let mut pos = start;
        while pos < end_exclusive && bytes[pos] != max_b {
            pos += 1;
        }
        debug_assert!(pos < end_exclusive);

        out.push(max_b);
        start = pos + 1;
        remaining -= 1;
    }

    String::from_utf8(out).expect("bank should be valid ASCII/UTF-8 digits")
}

pub fn total_joltage(banks: &[String], n: usize) -> i64 {
    banks
        .iter()
        .map(|bank| joltage(bank, n).parse::<i64>().expect("joltage should parse"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joltage_smoke_test() {
        assert_eq!(joltage("8647", 2), "87");
        assert_eq!(joltage("1119", 2), "19");
        assert_eq!(joltage("811111111111119", 2), "89");
        assert_eq!(joltage("818181911112111", 5), "92111");
        assert_eq!(joltage("818181911112111", 12), "888911112111");
    }
}
