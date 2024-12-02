use aoc24::read_day_input;
use itertools::Itertools;

fn main() {
    // parse in level reports
    let level_reports: Vec<Vec<i32>> = read_day_input(2)
        .lines() // split report lines
        .map(|report| {
            report // for each report
                .split_whitespace() // split on whitespace
                .map(|s| s.parse().unwrap()) // try to parse each string as an i32
                .collect()
        })
        .collect();

    // for each report, compare each level to the previous and emit the difference
    let level_diffs: Vec<Vec<i32>> = level_reports
        .iter()
        .map(|report| report.iter().tuple_windows().map(|(a, b)| b - a).collect()) // tuple_windows() was a handy find!
        .collect();

    // println!("Part 1: {:?}", &level_diffs[0]);

    // Count the number of reports where the level_diffs are all of the same sign AND the level_diff absolute values are between 1 and 3
    let part1_count = level_diffs
        .iter()
        .filter(|report| report.iter().all(|&x| x > 0) || report.iter().all(|&x| x < 0))
        .filter(|report| report.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3))
        .count();

    println!("Part 1: {}", part1_count);

    let part2_count = level_reports
        .iter()
        // for each report, create a list containing the report and every combination of the report with 1 element deleted
        .map(|report| {
            let mut damped_reports = Vec::new();
            damped_reports.push(report.clone());
            damped_reports.extend(
                (0..report.len())
                    .map(|i| {
                        report
                            .iter()
                            .enumerate()
                            .filter(|(j, _)| *j != i)
                            .map(|(_, x)| *x)
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>(),
            );
            damped_reports
        })
        // .map(|report| {
        //     println!("{:?}", report);
        //     report
        // })
        // For each report, calculate the difference between each element
        .map(|reports|
                // for each report, calculate the difference between each element
                reports
                    .iter()
                    .map(|report| {
                        report
                            .iter()
                            .tuple_windows()
                            .map(|(a, b)| b - a)
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>())
        // select any reports which meet the criteria
        .filter(|reports| {
            reports
                .iter()
                .any(|report| is_all_same_sign(report) && is_between_1_and_3(report))
        })
        // .map(|report| {
        //     println!("{:?}", report);
        //     report
        // })
        .count();

    println!("Part 2: {}", part2_count);
}

fn is_all_same_sign(vec: &Vec<i32>) -> bool {
    vec.iter().all(|&x| x > 0) || vec.iter().all(|&x| x < 0)
}

fn is_between_1_and_3(vec: &Vec<i32>) -> bool {
    vec.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3)
}
