use std::{env, fs};

/// Read file from ../data/dayXX.txt as a string
/// 
/// Given "1" will try to return a String with the contents of ../data/day01.txt
pub fn read_day_input(day: u32) -> String {
    fs::read_to_string(filepath(day)).expect("Should have been able to read input file")
}

fn filepath(day: u32) -> String {
    let day = format!("{:02}", day);
    let filename = format!("day{}.txt", day);
    let cwd = env::current_dir().unwrap();
    let parent = cwd.parent().unwrap();
    let filepath = parent.join("data").join(filename);
    filepath.to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filepath_with_leading_zero() {
        let result = filepath(1);
        assert_eq!(
            result,
            "/Users/craig/Code/github.com/craigjperry2/adventofcode/2024/data/day01.txt"
        );
    }

    #[test]
    fn filepath_without_leading_zero() {
        let result = filepath(10);
        assert_eq!(
            result,
            "/Users/craig/Code/github.com/craigjperry2/adventofcode/2024/data/day10.txt"
        );
    }

    #[test]
    #[should_panic]
    fn non_existent_file() {
        read_day_input(0);
    }
}
