use aoc24::read_day_input;
use std::collections::{BinaryHeap, HashMap};
use std::convert::Infallible;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(16);
    let grid: Grid = input.parse().expect("Failed to parse grid");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let score = part1(&grid);
    println!(
        "Part 1: '{score}' took {}µs",
        sw_part1.elapsed().as_micros()
    );
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid) -> isize {
    let mut path: Vec<Point> = Vec::new();
    let mut path_score: isize = 0;
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut from_score: HashMap<Point, isize> = HashMap::new();

    from_score.insert(grid.start, 0);
    open_set.push(Step {
        position: grid.start,
        previous_position: grid.start.next_point(-1, 0), // Start Facing East
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

            let is_different_direction = !current.position.in_line(current.previous_position, neighbor);
            let tentative_g_score = from_score[&current.position] + 1 + (1000 * is_different_direction as isize);
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

    path_score
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
        let mut grid: Vec<Cell> = Vec::new();
        let mut width: isize = 0;
        let mut height: isize = 0;
        let mut start: Point = Point::new(0, 0);
        let mut end: Point = Point::new(0, 0);

        for line in s.lines() {
            if width == 0 {
                width = isize::try_from(line.len()).unwrap();
            }

            for (i, c) in line.chars().enumerate() {
                let cell = Cell::from(c);
                if cell == Cell::Start {
                    start = Point::new(i, height as usize);
                }
                if cell == Cell::End {
                    end = Point::new(i, height as usize);
                }
                grid.push(cell);
            }

            height += 1;
        }

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
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Cell::*;

        match self {
            Empty => write!(f, "{}", Empty as u8 as char),
            Obstacle => write!(f, "{}", Obstacle as u8 as char),
            Start => write!(f, "{}", Start as u8 as char),
            End => write!(f, "{}", End as u8 as char),
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        use Cell::*;

        const EMPTY: u8 = Empty as u8;
        const OBSTACLE: u8 = Obstacle as u8;
        const START: u8 = Start as u8;
        const END: u8 = End as u8;

        match c as u8 {
            EMPTY => Empty,
            OBSTACLE => Obstacle,
            START => Start,
            END => End,
            _ => panic!("Invalid cell: {c}"),
        }
    }
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
        self.x == via.x && self.x == destination.x ||
            self.y == via.y && self.y == destination.y
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
        (other.score + other.heuristic).cmp(&(self.score + self.heuristic))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
