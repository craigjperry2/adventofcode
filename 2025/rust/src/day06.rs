use crate::core::Solution;
use color_eyre::eyre::Result;

pub struct Day06;

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String> {
        let worksheet: Vec<Vec<&str>> = input
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();
        let answer = grand_total(&worksheet);
        Ok(answer.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let worksheet: Vec<&str> = input.lines().collect();
        let answer = grand_total2(&worksheet);
        Ok(answer.to_string())
    }
}

fn grand_total(worksheet: &Vec<Vec<&str>>) -> i64 {
    let width = worksheet[0].len();
    let height = worksheet.len();
    let mut total = 0i64;
    for col in 0..width {
        let op = worksheet[height - 1][col];
        let nums = worksheet[..height - 1]
            .iter()
            .map(|row| row[col].parse::<i64>().expect("invalid integer"));
        let column_value = match op {
            "+" => nums.sum::<i64>(),
            "*" => nums.fold(1i64, |acc, x| acc * x),
            _ => panic!("unknown operator: {op:?}"),
        };
        total += column_value;
    }
    total
}

fn grand_total2(worksheet: &[&str]) -> i64 {
    let columns = break_into_columns(worksheet);
    let problems = transpose(columns);
    problems.iter().map(|p| cephalopodish_math(p)).sum()
}

fn break_into_columns(worksheet: &[&str]) -> Vec<Vec<String>> {
    let operator_line = worksheet.last().expect("worksheet is empty");
    let width = worksheet[0].len();
    let mut column_starts: Vec<usize> = operator_line
        .char_indices()
        .filter(|(_, ch)| *ch != ' ')
        .map(|(i, _)| i)
        .collect();
    column_starts.push(width + 1);
    worksheet
        .iter()
        .map(|line| break_line_into_columns(line, &column_starts))
        .collect()
}

fn break_line_into_columns(line: &str, column_starts: &[usize]) -> Vec<String> {
    let mut result = Vec::new();
    let padded_line = format!(
        "{:width$}",
        line,
        width = column_starts.last().unwrap_or(&0) + 1
    );
    for i in 0..column_starts.len() - 1 {
        let start = column_starts[i];
        let end = column_starts[i + 1] - 1;
        let slice = if start < padded_line.len() {
            let actual_end = std::cmp::min(end, padded_line.len());
            if start < actual_end {
                &padded_line[start..actual_end]
            } else {
                ""
            }
        } else {
            ""
        };
        result.push(slice.to_string());
    }
    result
}

fn cephalopodish_math(problem: &[String]) -> i64 {
    let (op, numbers) = problem.split_last().expect("problem is empty");
    let nums = vertically(numbers);
    match op.trim() {
        "+" => nums.iter().sum(),
        "*" => nums.iter().product(),
        _ => panic!("Unknown operator: {}", op),
    }
}

fn vertically(numbers: &[String]) -> Vec<i64> {
    let max_len = numbers.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut result = Vec::new();
    for i in 0..max_len {
        let mut digits = String::new();
        for num_str in numbers {
            if let Some(c) = num_str.chars().nth(i) {
                if !c.is_whitespace() {
                    digits.push(c);
                }
            }
        }
        if !digits.is_empty() {
            result.push(digits.parse::<i64>().unwrap_or(0));
        }
    }
    result
}

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return vec![];
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let example = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let worksheet: Vec<Vec<&str>> = example
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();
        let result = grand_total(&worksheet);
        assert!(result > 0);
    }

    #[test]
    fn test_vertically() {
        let rows = vec!["123".to_string(), " 45".to_string(), "  6".to_string()];
        assert_eq!(vertically(&rows), vec![1, 24, 356]);
    }

    #[test]
    fn test_part2_example() {
        let example = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let worksheet: Vec<&str> = example.lines().collect();
        assert_eq!(grand_total2(&worksheet), 3263827);
    }
}
