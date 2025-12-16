//! Composable parsing utilities inspired by Peter Norvig's AdventUtils (pytudes).
//!
//! Highlights:
//! - Section splitters: `lines`, `paragraphs`.
//! - Small parsers: `ints`, `positive_ints`, `digits`, `words`, `atom`, `atoms`.
//! - Helpers to read AoC input for a day and parse into records: `parse_day`, `parse_day_with`.
//! - Optional preview variants with debug printing: `parse_day_with_preview`, `parse_text_with_preview`.

use crate::core::read_or_fetch_input;
use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

/// A parsed atom: either a string, integer, or float.
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Str(String),
    Int(i64),
    Float(f64),
}

/// Split text into lines. Keeps empty lines and does not trim whitespace.
pub fn lines(text: &str) -> Vec<&str> {
    text.lines().collect()
}

/// Split text into paragraphs separated by a blank line ("\n\n").
/// Paragraph separators are not included in results.
pub fn paragraphs(text: &str) -> Vec<&str> {
    text.split("\n\n").collect()
}

/// Parse all sections from provided text using `parser` and `sections`.
pub fn parse_text<T>(text: &str, parser: fn(&str) -> T, sections: fn(&str) -> Vec<&str>) -> Vec<T> {
    sections(text.trim_end()).into_iter().map(parser).collect()
}

/// Variant that prints up to `show` input lines and parsed results (requires `T: Debug`).
pub fn parse_text_with_preview<T: Debug>(
    text: &str,
    parser: fn(&str) -> T,
    sections: fn(&str) -> Vec<&str>,
    show: usize,
) -> Vec<T> {
    let input_lines: Vec<&str> = text.lines().collect();
    show_items("Puzzle input", &input_lines, show);

    let records: Vec<T> = sections(text.trim_end()).into_iter().map(parser).collect();
    show_debug_items("Parsed representation", &records, show);
    records
}

/// Read the input for `day`, split it into `sections`, and apply `parser` to each. Returns records.
pub fn parse_day_with<T>(
    day: u8,
    parser: fn(&str) -> T,
    sections: fn(&str) -> Vec<&str>,
) -> Result<Vec<T>> {
    let text = read_or_fetch_input(day)?;
    Ok(parse_text(&text, parser, sections))
}

/// Convenience: parse day input with default `lines` sections.
pub fn parse_day<T>(day: u8, parser: fn(&str) -> T) -> Result<Vec<T>> {
    parse_day_with(day, parser, lines)
}

/// Preview variant that prints first `show` input lines and parsed records (requires `T: Debug`).
pub fn parse_day_with_preview<T: Debug>(
    day: u8,
    parser: fn(&str) -> T,
    sections: fn(&str) -> Vec<&str>,
    show: usize,
) -> Result<Vec<T>> {
    let text = read_or_fetch_input(day)?;
    Ok(parse_text_with_preview(&text, parser, sections, show))
}

/// Print up to `show` stringy items with a title. If `show == 0`, print nothing.
fn show_items<T: AsRef<str>>(title: &str, items: &[T], show: usize) {
    if show == 0 {
        return;
    }
    let n = items.len().min(show);
    eprintln!("{title} (first {n}):");
    for i in 0..n {
        eprintln!("{}", items[i].as_ref());
    }
    if items.len() > n {
        eprintln!("... and {} more", items.len() - n);
    }
}

/// Print up to `show` items using Debug.
fn show_debug_items<T: Debug>(title: &str, items: &[T], show: usize) {
    if show == 0 {
        return;
    }
    let n = items.len().min(show);
    eprintln!("{title} (first {n}):");
    for i in 0..n {
        eprintln!("{:?}", &items[i]);
    }
    if items.len() > n {
        eprintln!("... and {} more", items.len() - n);
    }
}

// -----------------------------
// Small composable parsers
// -----------------------------

static RE_INTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"-?[0-9]+").unwrap());
static RE_POSITIVE_INTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]+").unwrap());
static RE_DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]").unwrap());
static RE_WORDS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-Z]+").unwrap());
static RE_ATOMS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[+-]?\d+\.?\d*|\w+").unwrap());

/// All integers in `text` (negative allowed), ignoring non-number characters.
pub fn ints(text: &str) -> Vec<i64> {
    RE_INTS
        .find_iter(text)
        .filter_map(|m| m.as_str().parse::<i64>().ok())
        .collect()
}

/// All positive integers in `text`.
pub fn positive_ints(text: &str) -> Vec<u64> {
    RE_POSITIVE_INTS
        .find_iter(text)
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect()
}

/// All single digits in `text` as integers 0–9.
pub fn digits(text: &str) -> Vec<u8> {
    RE_DIGITS
        .find_iter(text)
        .filter_map(|m| m.as_str().parse::<u8>().ok())
        .collect()
}

/// All alphabetic words in `text`.
pub fn words(text: &str) -> Vec<String> {
    RE_WORDS
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Parse `text` into a single `Atom` (float or int or str). Trims whitespace.
pub fn atom(text: &str) -> Atom {
    let t = text.trim();
    if let Ok(x) = t.parse::<f64>() {
        if x.fract() == 0.0 {
            Atom::Int(x as i64)
        } else {
            Atom::Float(x)
        }
    } else {
        Atom::Str(t.to_string())
    }
}

/// All atoms (numbers or identifiers) in `text`. Skips punctuation.
pub fn atoms(text: &str) -> Vec<Atom> {
    RE_ATOMS.find_iter(text).map(|m| atom(m.as_str())).collect()
}

// -----------------------------
// Grid utilities (inspired by Norvig's AdventUtils Grid)
// -----------------------------

/// A 2D point `(x, y)` with `x` increasing to the right and `y` increasing downwards.
pub type Point = (i32, i32);

/// 4-connected neighbor directions: right, down, left, up.
pub const DIRECTIONS4: [Point; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// 8-connected neighbor directions (includes diagonals).
pub const DIRECTIONS8: [Point; 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];

/// Add a vector `d` to point `p`.
pub fn add2(p: Point, d: Point) -> Point {
    (p.0 + d.0, p.1 + d.1)
}

/// Missing-value behavior for `Grid` lookups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingChar {
    /// Mimic Python `KeyError`: accessing a missing point is considered an error; methods that
    /// fetch values will not synthesize a cell and will typically skip or return `None`.
    Error,
    /// Mimic Python `None` default: accessing a missing point returns no value.
    NoneValue,
    /// Use a concrete character for any missing cell (including off-grid).
    Value(char),
}

/// A 2D grid of characters, implemented as a mapping of `{(x, y): ch}`.
///
/// Coordinates are zero-based. Size is `(width, height)`.
#[derive(Debug, Clone)]
pub struct Grid {
    pub size: (i32, i32),
    cells: HashMap<Point, char>,
    directions: Vec<Point>,
    missing: MissingChar,
}

impl Grid {
    /// Initialize from an iterator of cells and an explicit size. Any out-of-range cells are kept,
    /// but `in_range` uses the provided `size`.
    pub fn from_cells<I>(
        size: (i32, i32),
        cells: I,
        directions: &[Point],
        missing: MissingChar,
    ) -> Self
    where
        I: IntoIterator<Item = (Point, char)>,
    {
        Grid {
            size,
            cells: cells.into_iter().collect(),
            directions: directions.to_vec(),
            missing,
        }
    }

    /// Initialize from a string (lines). Supports `skip` set and missing/default behavior.
    /// Width is the maximum line length; height is the number of lines. Cells with characters in
    /// `skip` are omitted from the mapping.
    pub fn from_str_with(
        text: &str,
        directions: &[Point],
        skip: &HashSet<char>,
        missing: MissingChar,
    ) -> Self {
        let lines: Vec<&str> = text.split('\n').collect();
        let height = lines.len() as i32;
        let width = lines
            .iter()
            .map(|l| l.chars().count() as i32)
            .max()
            .unwrap_or(0);
        let mut cells: HashMap<Point, char> = HashMap::new();
        for (y, row) in lines.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if !skip.contains(&ch) {
                    cells.insert((x as i32, y as i32), ch);
                }
            }
        }
        Grid {
            size: (width, height),
            cells,
            directions: directions.to_vec(),
            missing,
        }
    }

    /// Initialize from a string with 4-neighborhood, no skip, and `None`-like missing.
    pub fn from_str(text: &str) -> Self {
        Self::from_str_with(text, &DIRECTIONS4, &HashSet::new(), MissingChar::NoneValue)
    }

    /// Return a copy of the grid with the same directions and missing behavior.
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Is the point within the range of the grid's size?
    pub fn in_range(&self, p: Point) -> bool {
        0 <= p.0 && p.0 < self.size.0 && 0 <= p.1 && p.1 < self.size.1
    }

    /// Follow a ray starting at `start`, stepping by `direction`, yielding points until out of range.
    pub fn follow_line(&self, start: Point, direction: Point) -> Vec<Point> {
        let mut pts = Vec::new();
        let mut cur = start;
        while self.in_range(cur) {
            pts.push(cur);
            cur = add2(cur, direction);
        }
        pts
    }

    /// Points on the grid that neighbour `point` according to this grid's directions.
    /// Includes points that are not present in `cells` when `missing` is `Value(_)`.
    pub fn neighbours(&self, p: Point) -> Vec<Point> {
        let synthesize_missing = matches!(self.missing, MissingChar::Value(_));
        self.directions
            .iter()
            .map(|&d| add2(p, d))
            .filter(|q| self.cells.contains_key(q) || synthesize_missing)
            .collect()
    }

    /// The contents of the neighbouring points, using missing/default behavior.
    pub fn neighbour_contents(&self, p: Point) -> Vec<char> {
        self.neighbours(p)
            .into_iter()
            .filter_map(|q| self.get(q))
            .collect()
    }

    /// Get the cell value at `p`, applying the grid's missing policy.
    pub fn get(&self, p: Point) -> Option<char> {
        if let Some(&ch) = self.cells.get(&p) {
            return Some(ch);
        }
        match self.missing {
            MissingChar::Error => None,
            MissingChar::NoneValue => None,
            MissingChar::Value(c) => Some(c),
        }
    }

    /// Set the cell value at `p`.
    pub fn set(&mut self, p: Point, ch: char) {
        self.cells.insert(p, ch);
    }

    /// All points that contain one of the given characters.
    pub fn find_all<C: IntoIterator<Item = char>>(&self, contents: C) -> Vec<Point> {
        let set: HashSet<char> = contents.into_iter().collect();
        self.cells
            .iter()
            .filter_map(|(p, &ch)| if set.contains(&ch) { Some(*p) } else { None })
            .collect()
    }

    /// The contents of the grid as a rectangular list of rows. You can define a window
    /// with `xrange` and `yrange`; or they default to the whole grid. For missing == Error/None,
    /// the default fill is space `' '`; otherwise it is the provided `Value`.
    pub fn to_rows(
        &self,
        xrange: Option<std::ops::Range<i32>>,
        yrange: Option<std::ops::Range<i32>>,
    ) -> Vec<Vec<char>> {
        let xr = xrange.unwrap_or(0..self.size.0);
        let yr = yrange.unwrap_or(0..self.size.1);
        let default = match self.missing {
            MissingChar::Value(c) => c,
            MissingChar::Error | MissingChar::NoneValue => ' ',
        };
        yr.map(|y| {
            xr.clone()
                .map(|x| self.get((x, y)).unwrap_or(default))
                .collect()
        })
        .collect()
    }
}

/// Neighbours of this point using the given directions (can be used outside of a Grid).
pub fn neighbours(point: Point, directions: &[Point]) -> Vec<Point> {
    directions.iter().map(|&d| add2(point, d)).collect()
}
