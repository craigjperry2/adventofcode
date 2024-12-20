use crate::Cell::End;
use aoc24::read_day_input;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};
use std::convert::Infallible;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(18);
    let grid: Grid = input.parse().expect("Failed to parse grid");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let steps = part1(&grid, &input.lines().take(1024).join("\n"));
    println!(
        "Part 1: '{steps}' took {}µs",
        sw_part1.elapsed().as_micros()
    );

    let sw_part2 = std::time::Instant::now();
    let p = part2(&grid, &input);
    println!("Part 2: '{p:?}' took {}µs", sw_part2.elapsed().as_micros());
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid, s: &String) -> isize {
    let mut grid = grid.clone();

    s.lines()
        .map(|l| {
            l.split_once(',')
                .unwrap_or_else(|| panic!("Failed to split {l}"))
        })
        .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
        .for_each(|(x, y)| grid.grid[(y * grid.height + x) as usize] = Cell::Obstacle);

    let mut path: Vec<Point> = Vec::new();
    let mut path_score: isize = 0;
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut from_score: HashMap<Point, isize> = HashMap::new();

    from_score.insert(grid.start, 0);
    open_set.push(Step {
        position: grid.start,
        previous_position: grid.start,
        score: 0,
        heuristic: grid.start.distance_heuristic(grid.end),
    });

    while let Some(current) = open_set.pop() {
        if current.position == grid.end {
            path_score = current.score;
            path.push(current.position);
            let mut pos = current.position;
            while let Some(&parent) = came_from.get(&pos) {
                path.push(parent);
                pos = parent;
            }
            path.reverse();
            break;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let neighbor = current.position.next_point(*dx, *dy);

            if grid.is_out_of_bounds(&neighbor) || grid.is_obstacle(&neighbor) {
                continue;
            }

            let tentative_g_score = from_score[&current.position] + 1;
            if tentative_g_score < *from_score.get(&neighbor).unwrap_or(&isize::MAX) {
                came_from.insert(neighbor, current.position);
                from_score.insert(neighbor, tentative_g_score);
                open_set.push(Step {
                    position: neighbor,
                    previous_position: current.position,
                    score: tentative_g_score,
                    heuristic: neighbor.distance_heuristic(grid.end),
                });
            }
        }
    }

    path.iter()
        .for_each(|p| grid.grid[p.to_offset(grid.width)] = Cell::Path);

    path_score
}

// -------------------- PART 2 --------------------

fn part2(grid: &Grid, s: &String) -> usize {
    let mut result = 0;
    for i in (0..3450).rev() {
        if part1(grid, &s.lines().take(1023 + i).join("\n")) == 0 {
            result = 1023 + i;
            break
        }
    }
    result
}

// -------------------- TYPES: GRID --------------------

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<Cell>,
    width: isize,
    height: isize,
    start: Point,
    end: Point,
}

impl Grid {
    fn is_out_of_bounds(&self, p: &Point) -> bool {
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }

    fn is_obstacle(&self, p: &Point) -> bool {
        let c = &self.grid[p.to_offset(self.width)];
        *c == Cell::Obstacle
    }
}

impl FromStr for Grid {
    type Err = Infallible; // Cell parsing will just panic

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width: isize = 71;
        let height: isize = 71;
        let start: Point = Point::new(0, 0);
        let end: Point = Point {
            x: width - 1,
            y: height - 1,
        };
        let mut grid: Vec<Cell> = vec![Cell::Empty; (width * height) as usize];

        Ok(Self {
            grid,
            width,
            height,
            start,
            end,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x as usize, y as usize);
                let cell = &self.grid[p.to_offset(self.width)];
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// -------------------- TYPES: CELL --------------------

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Cell {
    Empty = b'.',
    Obstacle = b'#',
    Start = b'S',
    End = b'E',
    Path = b'O',
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Cell::*;

        match self {
            Empty => write!(f, "{}", Empty as u8 as char),
            Obstacle => write!(f, "{}", Obstacle as u8 as char),
            Start => write!(f, "{}", Start as u8 as char),
            End => write!(f, "{}", End as u8 as char),
            Path => write!(f, "{}", Path as u8 as char),
        }
    }
}

// -------------------- TYPES: POINT --------------------

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, PartialOrd, Ord)]
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

    fn to_offset(&self, width: isize) -> usize {
        (self.y * width + self.x) as usize
    }

    fn distance_heuristic(&self, destination: Point) -> isize {
        (self.x - destination.x).abs() + (self.y - destination.y).abs()
    }

    fn next_point(&self, dx: isize, dy: isize) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn in_line(&self, via: Point, destination: Point) -> bool {
        self.x == via.x && self.x == destination.x || self.y == via.y && self.y == destination.y
    }
}

// -------------------- TYPES: STEP --------------------

#[derive(Debug, Clone, Eq, PartialEq)]
struct Step {
    position: Point,
    previous_position: Point,
    score: isize,
    heuristic: isize,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.score + other.heuristic + other.position.y + other.position.x)
            .cmp(&(self.score + self.heuristic + self.position.y + self.position.x))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
