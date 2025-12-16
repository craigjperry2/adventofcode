use crate::core::{read_or_fetch_input, Solution};
use crate::parsers::{Grid, MissingChar, Point, DIRECTIONS8};
use color_eyre::eyre::Result;

pub struct Day04;

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String> {
        let paper_grid = Grid::from_str_with(
            &read_or_fetch_input(4)?,
            &DIRECTIONS8,
            &Default::default(),
            MissingChar::Error,
        );
        let rolls = accessible_rolls(paper_grid);
        Ok(rolls.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let paper_grid = Grid::from_str_with(
            &read_or_fetch_input(4)?,
            &DIRECTIONS8,
            &Default::default(),
            MissingChar::Error,
        );
        let rolls = removable_rolls(paper_grid);
        Ok(rolls.len().to_string())
    }
}

/// A roll of paper is accessible if there are fewer than
/// four rolls of paper in the eight adjacent positions.
fn is_accessible(point: Point, grid: Grid) -> bool {
    grid.get(point).unwrap() == '@'
        && grid
            .neighbour_contents(point)
            .iter()
            .filter(|&x| *x == '@')
            .count()
            < 4
}

/// The positions of all the accessible rolls of paper on the grid.
fn accessible_rolls(grid: Grid) -> Vec<Point> {
    grid.find_all(['@'])
        .into_iter()
        .filter(|&p| is_accessible(p, grid.copy()))
        .collect()
}

fn removable_rolls(grid: Grid) -> Vec<Point> {
    let mut grid2 = grid.copy();
    let mut q = grid.find_all(['@']);
    let mut removed = Vec::new();
    while let Some(p) = q.pop() {
        if is_accessible(p, grid2.copy()) {
            removed.push(p);
            grid2.set(p, '.');
            q.extend(grid2.neighbours(p));
        }
    }
    removed
}
