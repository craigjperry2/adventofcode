use aoc24::read_day_input;
use std::collections::HashSet;
use std::convert::Infallible;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let grid: Grid = read_day_input(6).parse().expect("Failed to parse grid");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let distinct_positions = part1(&grid).len();
    println!(
        "Part 1: '{distinct_positions}' took {}ms",
        sw_part1.elapsed().as_millis()
    );

    let sw_part2 = std::time::Instant::now();
    let positions = part2(&grid).len();
    println!(
        "Part 2: '{positions}' took {}ms",
        sw_part2.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid) -> HashSet<Point> {
    let start = find_start(grid);
    recursive_step_through_grid(grid, &start, HashSet::new())
}

fn find_start(grid: &Grid) -> Guard {
    let index: usize = grid
        .cells
        .iter()
        .position(|c| *c == Cell::StartPosition)
        .expect("Did not find a starting position");

    Guard {
        p: Point::from_index(grid.width, index),
        d: Direction::North,
    }
}

fn recursive_step_through_grid(grid: &Grid, loc: &Guard, mut visited: HashSet<Point>) -> HashSet<Point> {
    if grid.is_out_of_bounds(&loc.p) {
        return visited;
    }

    visited.insert(loc.p);

    let next_loc = next_location(grid, loc);
    recursive_step_through_grid(grid, &next_loc, visited)
}

fn next_location(grid: &Grid, loc: &Guard) -> Guard {
    let next_p = loc.p.step(&loc.d);
    if !grid.is_out_of_bounds(&next_p) && grid.is_obstacle(&next_p) {
        // Obstacle ahead, turn right 90 but don't move forward
        Guard {
            p: loc.p,
            d: loc.d.turn_right(),
        }
    } else {
        // Move one forward in same direction
        Guard {
            p: next_p,
            d: loc.d.clone(),
        }
    }
}

// -------------------- PART 2 --------------------

fn part2(grid: &Grid) -> Vec<Grid> {
    let start = find_start(grid);

    candidate_grids(grid)
        .into_iter()
        .filter(|g| has_loop(g, &start))
        .collect()
}

fn candidate_grids(grid: &Grid) -> Vec<Grid> {
    let part1_journey: HashSet<usize> = part1(grid)
        .iter()
        .map(|p| p.to_index(grid.width))
        .collect();

    (0..grid.cells.len())
        // .filter(|i| grid.cells[*i] == Cell::Empty)
        .filter(|i| part1_journey.contains(i))
        .map(|i| {
            let mut g = grid.cells.clone();

            g[i] = Cell::Obstacle;
            Grid {
                cells: g,
                width: grid.width,
                height: grid.height,
            }
        })
        .collect()
}

fn has_loop(grid: &Grid, start: &Guard) -> bool {
    let mut seen = HashSet::new();
    let mut loc = start.clone();

    loop {
        if grid.is_out_of_bounds(&loc.p) {
            return false;
        }

        let next_loc = next_location(grid, &loc);
        if next_loc.d != loc.d && seen.contains(&loc) {
            return true;
        }
        if next_loc.d != loc.d {
            seen.insert(loc.clone());
        }

        loc = next_loc;
    }
}

// -------------------- TYPES: CELL --------------------

#[derive(Clone, PartialEq, Eq)]
#[repr(u8)]
enum Cell {
    Empty = b'.',
    Obstacle = b'#',
    StartPosition = b'^',
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Cell::*;

        match self {
            Empty => write!(f, "{}", Empty as u8 as char),
            Obstacle => write!(f, "{}", Obstacle as u8 as char),
            StartPosition => write!(f, "{}", StartPosition as u8 as char),
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        use Cell::*;
        
        const EMPTY: u8 = Empty as u8;
        const OBSTACLE: u8 = Obstacle as u8;
        const START_POSITION: u8 = StartPosition as u8;

        match c as u8 {
            EMPTY => Empty,
            OBSTACLE => Obstacle,
            START_POSITION => StartPosition,
            _ => panic!("Invalid cell: {c}"),
        }
    }
}

// -------------------- TYPES: GRID --------------------

#[derive(Clone)]
struct Grid {
    cells: Vec<Cell>,
    width: isize,
    height: isize,
}

impl Grid {
    fn is_out_of_bounds(&self, p: &Point) -> bool {
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }

    fn is_obstacle(&self, p: &Point) -> bool {
        self.cells[p.to_index(self.width)] == Cell::Obstacle
    }
}

impl FromStr for Grid {
    type Err = Infallible; // Cell parsing will just panic

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::new();
        let mut width: isize = 0;
        let mut height: isize = 0;

        for line in s.lines() {
            if width == 0 {
                width = isize::try_from(line.len()).unwrap();
            }

            for c in line.chars() {
                cells.push(Cell::from(c));
            }

            height += 1;
        }

        Ok(Self {
            cells,
            width,
            height,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[usize::try_from(y * self.width + x).unwrap()];
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// -------------------- TYPES: DIRECTION --------------------

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        use Direction::*;

        match self {
            North => East,
            East => South,
            South => West,
            West => North,
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
    fn step(&self, d: &Direction) -> Self {
        use Direction::*;

        match d {
            North => Self {
                x: self.x,
                y: self.y - 1,
            },
            East => Self {
                x: self.x + 1,
                y: self.y,
            },
            South => Self {
                x: self.x,
                y: self.y + 1,
            },
            West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn from_index(width: isize, uindex: usize) -> Self {
        let iindex = isize::try_from(uindex).unwrap();

        Self {
            x: iindex % width,
            y: iindex / width,
        }
    }

    fn to_index(&self, width: isize) -> usize {
        usize::try_from(self.y * width + self.x).unwrap()
    }
}

// -------------------- TYPES: GUARD --------------------

#[derive(PartialEq, Eq, Clone, Hash)]
struct Guard {
    p: Point,
    d: Direction,
}
