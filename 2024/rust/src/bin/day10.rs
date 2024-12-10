use aoc24::read_day_input;
use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let grid: Grid = read_day_input(10).parse().expect("Failed to parse grid");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let trailheads_sum = part1(&grid);
    println!(
        "Part 1: '{trailheads_sum}' took {}µs",
        sw_part1.elapsed().as_micros()
    );
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid) -> usize {
    find_all_trail_starts(grid)
        .iter()
        // .inspect(|&trailhead| println!("starting: {trailhead:?}"))
        .map(|&start| {
            let mut trails: Vec<Vec<Point>> = Vec::new();
            find_all_trailheads(grid, start, &Vec::new(), &mut trails);
            trails
        }) 
        .flat_map(|trail| trail.iter().map(|path| path[8]).collect::<HashSet<Point>>()) // unique trailheads
        // .inspect(|&p| println!("  Found trailhead at: {p:?}"))
        .collect::<Vec<Point>>()
        .len()
}

fn find_all_trail_starts(grid: &Grid) -> Vec<Point> {
    grid.rows
        .iter()
        .enumerate()
        .flat_map(|(y, cols)| {
            cols.iter()
                .enumerate()
                .filter(|(_, &c)| c == 0)
                .map(|(x, _)| Point::new(x, y))
                .collect::<Vec<Point>>()
        })
        .collect()
}

fn find_all_trailheads<'a>(
    grid: &Grid,
    loc: Point,
    steps: &Vec<Point>,
    mut paths: &'a mut Vec<Vec<Point>>,
) {
    if grid.is_out_of_bounds(&loc) {
        return;
    }

    if grid.terrain_height(loc) == 9 {
        paths.push(steps.clone());
        return;
    }

    for dir in [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        let next = loc.step(&dir);
        if !grid.is_out_of_bounds(&next)
            && grid.terrain_height(next) - grid.terrain_height(loc) == 1
            && !steps.contains(&next)
        {
            let mut steps = steps.clone();
            steps.push(next);
            find_all_trailheads(grid, next, &steps, &mut paths);
        }
    }
}

// -------------------- TYPES: GRID --------------------

#[derive(Clone)]
struct Grid {
    rows: Vec<Vec<u32>>,
    width: isize,
    height: isize,
}

impl Grid {
    fn is_out_of_bounds(&self, p: &Point) -> bool {
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }

    fn terrain_height(&self, p: Point) -> i32 {
        self.rows[p.y as usize][p.x as usize] as i32
    }
}

impl FromStr for Grid {
    type Err = Infallible; // Cell parsing will just panic

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        let mut width: isize = 0;
        let mut height: isize = 0;

        for line in s.lines() {
            if width == 0 {
                width = isize::try_from(line.len()).unwrap();
            }

            rows.push(
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .unwrap_or_else(|| panic!("Failed to parse {c}"))
                    })
                    .collect(),
            );

            height += 1;
        }

        Ok(Self {
            rows,
            width,
            height,
        })
    }
}

// -------------------- TYPES: DIRECTION --------------------

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

// -------------------- TYPES: POINT --------------------

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: isize::try_from(x).unwrap(),
            y: isize::try_from(y).unwrap(),
        }
    }

    fn step(&self, d: &Direction) -> Self {
        match d {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}
