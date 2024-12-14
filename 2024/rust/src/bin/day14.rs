use std::convert::Infallible;
use aoc24::read_day_input;
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let robots: Vec<Robot> = read_day_input(14).lines().map(|l| l.parse().expect("Failed to parse robots")).collect();
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let safety_factor = part1(&robots);
    println!(
        "Part 1: '{safety_factor}' took {}µs",
        sw_part1.elapsed().as_micros()
    );
}

// -------------------- PART 1 --------------------

fn part1(robots: &Vec<Robot>) -> i32 {
    const WIDTH: i32 = 101;
    const MID_X: i32 = WIDTH / 2;
    const HEIGHT: i32 = 103;
    const MID_Y: i32 = HEIGHT / 2;

    let mut updated_robots: Vec<Robot> = robots.clone();

    for _ in 0..100 {
        for robot in &mut updated_robots {
            robot.x = (robot.x + robot.dx).rem_euclid(WIDTH);
            robot.y = (robot.y + robot.dy).rem_euclid(HEIGHT);
        }
    }

    let mut top_left = 0; // Top-left
    let mut top_right = 0; // Top-right
    let mut bottom_left = 0; // Bottom-left
    let mut bottom_right = 0; // Bottom-right

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

#[derive(Debug, Clone)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl FromStr for Robot {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_comma = s.find(',').unwrap();
        let space = s.find(' ').unwrap();
        let last_equals = s.rfind('=').unwrap();
        let last_comma = s.rfind(',').unwrap();

        Ok(Self {
            x: s[2..first_comma].parse().unwrap(),
            y: s[first_comma + 1..space].parse().unwrap(),
            dx: s[last_equals + 1..last_comma].parse().unwrap(),
            dy: s[last_comma + 1..].parse().unwrap(),
        })
    }
}
