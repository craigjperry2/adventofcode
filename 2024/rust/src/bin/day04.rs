use aoc24::read_day_input;
use std::collections::HashMap;
use std::convert::TryFrom;

fn main() {
    let grid: Vec<Vec<char>> = read_day_input(4)
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut start_locations: Vec<(Point, Direction)> = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let coord = Point { x: x, y: y };
            search_all_directions(&grid, coord, String::from("XMAS"), &mut start_locations);
        }
    }

    println!("Part 1: {:?}", start_locations.len());

    let mut xstart_locations: Vec<(Point, Direction)> = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let coord = Point { x: x, y: y };
            search_x_directions(&grid, coord, String::from("MAS"), &mut xstart_locations);
        }
    }

    // iterate over xstart locations and calculate the coord of A
    let a_locations: Vec<Point> = xstart_locations
        .iter()
        .map(|(p, d)| Point {
            x: p.x.checked_add_signed(d.x_offset()).expect("invalid a x"),
            y: p.y.checked_add_signed(d.y_offset()).expect("invalid a y"),
        })
        .collect();

    // count number of duplicates in a_locations
    let mut freq = HashMap::new();
    for loc in &a_locations {
        *freq.entry(*loc).or_insert(0) += 1;
    }

    let duplicate_count = freq.values().filter(|&&count| count > 1).count();

    println!("Part 2: {:?}", duplicate_count);
}

fn search_all_directions(
    grid: &Vec<Vec<char>>,
    coord: Point,
    needle: String,
    locations: &mut Vec<(Point, Direction)>,
) {
    let (width, height) = (
        isize::try_from(grid[0].len()).expect("grid width too large"),
        isize::try_from(grid.len()).expect("grid height too large"),
    );

    let (mut curr_x, mut curr_y) = (
        isize::try_from(coord.x).expect("invalid x starting location"),
        isize::try_from(coord.y).expect("invalid y starting location"),
    );

    let c = needle.chars().nth(0).expect("empty needle");
    let g = grid[curr_y as usize][curr_x as usize];
    if c != g {
        return;
    }

    for d in Direction::VALUES {
        curr_x = isize::try_from(coord.x).expect("invalid starting x") + d.x_offset();
        curr_y = isize::try_from(coord.y).expect("invalid starting y") + d.y_offset();

        // for len word starting from 1 (because early return)
        for w in 1..needle.len() {
            if curr_x < 0 || curr_x >= width || curr_y < 0 || curr_y >= height {
                break;
            }

            let c = needle.chars().nth(w).expect("overflowed needle");
            let g = grid[curr_y as usize][curr_x as usize];
            if c != g {
                break;
            }

            curr_x += d.x_offset();
            curr_y += d.y_offset();

            if w == needle.len() - 1 {
                locations.push((coord, d));
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    const VALUES: [Self; 8] = [
        Direction::N,
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::S,
        Direction::SW,
        Direction::W,
        Direction::NW,
    ];

    const XVALUES: [Self; 4] = [Direction::NE, Direction::SE, Direction::SW, Direction::NW];

    fn x_offset(&self) -> isize {
        match self {
            Direction::N => 0,
            Direction::NE => 1,
            Direction::E => 1,
            Direction::SE => 1,
            Direction::S => 0,
            Direction::SW => -1,
            Direction::W => -1,
            Direction::NW => -1,
        }
    }

    fn y_offset(&self) -> isize {
        match self {
            Direction::N => -1,
            Direction::NE => -1,
            Direction::E => 0,
            Direction::SE => 1,
            Direction::S => 1,
            Direction::SW => 1,
            Direction::W => 0,
            Direction::NW => -1,
        }
    }
}

fn search_x_directions(
    grid: &Vec<Vec<char>>,
    coord: Point,
    needle: String,
    locations: &mut Vec<(Point, Direction)>,
) {
    let (width, height) = (
        isize::try_from(grid[0].len()).expect("grid width too large"),
        isize::try_from(grid.len()).expect("grid height too large"),
    );

    let (mut curr_x, mut curr_y) = (
        isize::try_from(coord.x).expect("invalid x starting location"),
        isize::try_from(coord.y).expect("invalid y starting location"),
    );

    let c = needle.chars().nth(0).expect("empty needle");
    let g = grid[curr_y as usize][curr_x as usize];
    if c != g {
        return;
    }

    for d in Direction::XVALUES {
        curr_x = isize::try_from(coord.x).expect("invalid starting x") + d.x_offset();
        curr_y = isize::try_from(coord.y).expect("invalid starting y") + d.y_offset();

        // for len word starting from 1 (because early return)
        for w in 1..needle.len() {
            if curr_x < 0 || curr_x >= width || curr_y < 0 || curr_y >= height {
                break;
            }

            let c = needle.chars().nth(w).expect("overflowed needle");
            let g = grid[curr_y as usize][curr_x as usize];
            if c != g {
                break;
            }

            curr_x += d.x_offset();
            curr_y += d.y_offset();

            if w == needle.len() - 1 {
                locations.push((coord, d));
            }
        }
    }
}
