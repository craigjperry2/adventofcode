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
pub fn parse_day_with<T>(day: u8, parser: fn(&str) -> T, sections: fn(&str) -> Vec<&str>) -> Result<Vec<T>> {
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
    if show == 0 { return; }
    let n = items.len().min(show);
    eprintln!("{title} (first {n}):");
    for i in 0..n {
        eprintln!("{}", items[i].as_ref());
    }
    if items.len() > n { eprintln!("... and {} more", items.len() - n); }
}

/// Print up to `show` items using Debug.
fn show_debug_items<T: Debug>(title: &str, items: &[T], show: usize) {
    if show == 0 { return; }
    let n = items.len().min(show);
    eprintln!("{title} (first {n}):");
    for i in 0..n {
        eprintln!("{:?}", &items[i]);
    }
    if items.len() > n { eprintln!("... and {} more", items.len() - n); }
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
pub fn positive_ints(text: &str) -> Vec<i64> {
    RE_POSITIVE_INTS
        .find_iter(text)
        .filter_map(|m| m.as_str().parse::<i64>().ok())
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
        if x.fract() == 0.0 { Atom::Int(x as i64) } else { Atom::Float(x) }
    } else {
        Atom::Str(t.to_string())
    }
}

/// All atoms (numbers or identifiers) in `text`. Skips punctuation.
pub fn atoms(text: &str) -> Vec<Atom> {
    RE_ATOMS.find_iter(text).map(|m| atom(m.as_str())).collect()
}

