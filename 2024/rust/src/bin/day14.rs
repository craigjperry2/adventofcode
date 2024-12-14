use aoc24::read_day_input;
use std::cmp::{max, min};
use std::num::ParseIntError;
use std::str::FromStr;

const WIDTH: i32 = 101;
const MID_X: i32 = WIDTH / 2;
const HEIGHT: i32 = 103;
const MID_Y: i32 = HEIGHT / 2;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let robots: Vec<Robot> = read_day_input(14)
        .lines()
        .map(|l| l.parse().expect("Failed to parse robots"))
        .collect();
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let safety_factor = part1(&robots);
    println!(
        "Part 1: '{safety_factor}' took {}µs",
        sw_part1.elapsed().as_micros()
    );

    let sw_part2 = std::time::Instant::now();
    let seconds = part2(&robots);
    println!(
        "Part 2: '{seconds}' took {}ms",
        sw_part2.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(robots: &Vec<Robot>) -> i32 {
    let mut updated_robots: Vec<Robot> = robots.clone();

    for _ in 0..100 {
        for robot in &mut updated_robots {
            robot.x = (robot.x + robot.dx).rem_euclid(WIDTH);
            robot.y = (robot.y + robot.dy).rem_euclid(HEIGHT);
        }
    }

    // Quadrants
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in updated_robots {
        if robot.x == MID_X || robot.y == MID_Y {
            continue;
        }

        if robot.x < MID_X && robot.y < MID_Y {
            top_left += 1;
        } else if robot.x >= MID_X && robot.y < MID_Y {
            top_right += 1;
        } else if robot.x < MID_X && robot.y >= MID_Y {
            bottom_left += 1;
        } else if robot.x >= MID_X && robot.y >= MID_Y {
            bottom_right += 1;
        }
    }

    top_left * top_right * bottom_left * bottom_right
}

// REJECTED: i mis-read the problem, key word "MOST" of the robots form a tree, a global bounding
//           box doesn't work
// fn part2_global_bounding_box(robots: &Vec<Robot>) -> i32 {
//     let mut updated_robots: Vec<Robot> = robots.clone();
//     let mut smallest_width = WIDTH;
//     let mut smallest_height = HEIGHT;
//     let mut best_time = 0;
//     let mut best_positions = robots.clone();
//
//     for seconds in 1.. {
//         for robot in &mut updated_robots {
//             robot.x = (robot.x + robot.dx).rem_euclid(WIDTH);
//             robot.y = (robot.y + robot.dy).rem_euclid(HEIGHT);
//         }
//
//         // Bounding box
//         let mut x_min = WIDTH;
//         let mut x_max = 0;
//         let mut y_min = HEIGHT;
//         let mut y_max = 0;
//
//         for robot in &updated_robots {
//             x_min = min(x_min, robot.x);
//             x_max = max(x_max, robot.x);
//             y_min = min(y_min, robot.y);
//             y_max = max(y_max, robot.y);
//         }
//
//         let width = x_max - x_min;
//         let height = y_max - y_min;
//
//         smallest_width = min(smallest_width, width);
//         smallest_height = min(smallest_height, height);
//
//         if height = smallest_height {
//             best_time = seconds;
//             best_positions.clone_from(&updated_robots);
//         } else if height > smallest_height {
//             // The robots are spreading out again. Simulation is done.
//             break;
//         }
//     }
//
//     visualise(&best_positions);
//
//     best_time
// }
//
// fn visualise(robots: &[Robot], cluster: &HashSet<usize>) {
//     // Find the bounding box of the primary cluster
//     let mut x_min = i32::MAX;
//     let mut x_max = i32::MIN;
//     let mut y_min = i32::MAX;
//     let mut y_max = i32::MIN;
//
//     for &i in cluster {
//         x_min = min(x_min, robots[i].x);
//         x_max = max(x_max, robots[i].x);
//         y_min = min(y_min, robots[i].y);
//         y_max = max(y_max, robots[i].y);
//     }
//
//     let width = (x_max - x_min + 1) as usize;
//     let height = (y_max - y_min + 1) as usize;
//
//     let mut grid = vec![vec!['.'; width]; height];
//
//     for i in cluster {
//         let x = (robots[*i].x - x_min) as usize;
//         let y = (robots[*i].y - y_min) as usize;
//         grid[y][x] = '#';
//     }
//
//     for row in grid {
//         println!("{}", row.iter().collect::<String>());
//     }
// }

fn part2(robots: &Vec<Robot>) -> i32 {
    let mut updated_robots: Vec<Robot> = robots.clone();
    let mut best_time = 0;
    let mut best_positions = Vec::new();
    let mut best_cluster = Vec::new();

    for seconds in 1..10000 {
        for robot in &mut updated_robots {
            robot.x = (robot.x + robot.dx).rem_euclid(WIDTH);
            robot.y = (robot.y + robot.dy).rem_euclid(HEIGHT);
        }

        let cluster = find_cluster(&updated_robots);
        if cluster.is_empty() {
            continue;
        }

        if cluster.len() > best_cluster.len() {
            best_time = seconds;
            best_positions.clone_from(&updated_robots);
            best_cluster.clone_from(&cluster);
        }
    }

    visualise(&best_positions, &best_cluster);
    best_time
}

fn find_cluster(robots: &[Robot]) -> Vec<usize> {
    let mut cluster = Vec::new();
    let mut visited = vec![false; robots.len()];

    // time saving lookup table: grid position -> offset. Reduces 60s -> 1.5s
    let mut lookup: Vec<Option<usize>> = vec![None; (WIDTH * HEIGHT) as usize];
    for (i, robot) in robots.iter().enumerate() {
        lookup[(robot.x * HEIGHT + robot.y) as usize] = Some(i);
    }

    for i in 0..robots.len() {
        if visited[i] {
            continue;
        }

        let mut stack = vec![i];
        let mut current_cluster = Vec::new();

        while let Some(curr) = stack.pop() {
            if !visited[curr] {
                visited[curr] = true;
                current_cluster.push(curr);

                for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let candidate_x = (robots[curr].x + dx).rem_euclid(WIDTH);
                    let candidate_y = (robots[curr].y + dy).rem_euclid(HEIGHT);
                    if let Some(i) = &lookup[(candidate_x * HEIGHT + candidate_y) as usize] {
                        if !visited[*i] {
                            stack.push(*i);
                        }
                    }
                }
            }
        }

        if current_cluster.len() > cluster.len() {
            cluster = current_cluster;
        }
    }

    cluster
}

fn visualise(robots: &[Robot], cluster: &Vec<usize>) {
    let mut x_min = WIDTH;
    let mut x_max = 0;
    let mut y_min = HEIGHT;
    let mut y_max = 0;

    for &i in cluster {
        x_min = min(x_min, robots[i].x);
        x_max = max(x_max, robots[i].x);
        y_min = min(y_min, robots[i].y);
        y_max = max(y_max, robots[i].y);
    }

    let width = (x_max - x_min + 1) as usize;
    let height = (y_max - y_min + 1) as usize;

    let mut grid = vec![vec!['.'; width]; height];

    for &i in cluster {
        let x = (robots[i].x - x_min) as usize;
        let y = (robots[i].y - y_min) as usize;
        grid[y][x] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

#[derive(Debug, Clone)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl FromStr for Robot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_comma = s.find(',').unwrap();
        let space = s.find(' ').unwrap();
        let last_equals = s.rfind('=').unwrap();
        let last_comma = s.rfind(',').unwrap();

        Ok(Self {
            x: s[2..first_comma].parse()?,
            y: s[first_comma + 1..space].parse()?,
            dx: s[last_equals + 1..last_comma].parse()?,
            dy: s[last_comma + 1..].parse()?,
        })
    }
}
