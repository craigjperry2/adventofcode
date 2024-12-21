use aoc24::read_day_input;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::convert::Infallible;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(20);
    let grid: Grid = input.parse().expect("Failed to parse grid");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let cheats = part1(&grid);
    println!("Part 1: '{cheats}' took {}s", sw_part1.elapsed().as_secs());

    let sw_part2 = std::time::Instant::now();
    let cheats = part2(&grid);
    println!("Part 2: '{cheats}' took {}s", sw_part2.elapsed().as_secs());
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid) -> usize {
    let path = astar_race(grid);
    find_blocks_separating_parallel_paths(grid, &path)
        .par_iter()
        .map(|block| remove_block(grid, *block))
        .map(|r| astar_race(&r).len())
        .filter(|&picoseconds| (path.len() as isize - picoseconds as isize) > 99)
        .count()
}

// -------------------- PART 2 --------------------

fn part2(grid: &Grid) -> usize {
    let mut total = 0;
    let path = astar_race(grid);
    // println!("{path:?}");
    for start in 0..path.len() - 1 {
        for end in start + 1..path.len() {
            let distance = path[start].distance_heuristic(path[end]);
            let difference = (end - start) as isize - distance;
            if distance <= 20 && difference > 99 {
                // println!("s:{start} e:{end} dist:{distance} diff:{difference}");
                total += 1;
            }
        }
    }
    total
}

// 945714 too low
// 1078443 too high

fn find_blocks_separating_parallel_paths(grid: &Grid, path: &Vec<Point>) -> Vec<Point> {
    let mut separating_blocks = Vec::new();

    for i in 0..path.len() {
        for j in i + 2..path.len() {
            let p1 = path[i];
            let p2 = path[j];

            if p1.in_line(p2) {
                let potential_block = if p1.x == p2.x {
                    Point {
                        x: p1.x,
                        y: (p1.y + p2.y) / 2,
                    }
                } else {
                    Point {
                        x: (p1.x + p2.x) / 2,
                        y: p1.y,
                    }
                };

                if (p1.distance_heuristic(p2) == 2) && grid.is_obstacle(&potential_block) {
                    separating_blocks.push(potential_block);
                }
            }
        }
    }

    separating_blocks
}

fn remove_block(grid: &Grid, block: Point) -> Grid {
    let mut new_grid = grid.clone();
    new_grid.grid[block.to_offset(grid.width)] = Cell::Empty;
    new_grid
}

fn astar_race(grid: &Grid) -> Vec<Point> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(grid.start, 0);
    f_score.insert(grid.start, grid.start.distance_heuristic(grid.end));
    open_set.push((Reverse(grid.start.distance_heuristic(grid.end)), grid.start));

    while let Some((_, current)) = open_set.pop() {
        if current == grid.end {
            let mut path: Vec<Point> = vec![grid.end];
            let mut current = grid.end;
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
            }
            return path;
        }

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let neighbor = current.next_point(dx, dy);

            if !grid.is_out_of_bounds(&neighbor) && !grid.is_obstacle(&neighbor) {
                let tentative_g_score = g_score[&current] + 1;

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&isize::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    let f = tentative_g_score + neighbor.distance_heuristic(grid.end);
                    f_score.insert(neighbor, f);
                    open_set.push((Reverse(f), neighbor));
                }
            }
        }
    }

    panic!("Removing a block failed to find path!");
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

    fn in_line(&self, destination: Point) -> bool {
        self.x == destination.x || self.y == destination.y
    }
}
