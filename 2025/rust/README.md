# Advent of Code 2025 (Rust)

This repo contains a small Rust scaffold to organize, run and submit Advent of Code 2025 solutions for craigjperry2.

## Highlights
- CLI via `clap` with commands: `run`, `fetch`, and `input-path`.
- Developer experience: `color-eyre` (pretty errors), `anyhow` (Result ergonomics), `dotenvy` (loads `.env`).
- Core utilities:
  - `Solution` trait for per-day solvers (`part1`/`part2`).
  - Input caching in `inputs/dayXX.txt` and helpers to read/fetch.
  - Optional online integration behind feature `online` using blocking `reqwest` to fetch inputs and submit answers with your `AOC_SESSION` cookie.
- Offline by default — HTTP only when built with `--features online`.

---

# Quick start

Prerequisites: Rust toolchain (stable). Optionally `direnv` if you like auto-loading env vars.

```bash
# Build & see help
cargo run -- --help

# Run a day (expects inputs/day01.txt to exist)
cargo run -- run 1          # tries part1 then part2 if implemented
cargo run -- run 1 p1       # run only part 1
cargo run -- run 1 p2       # run only part 2
```

---

# Online features (optional)

Network access is opt-in. Enable it with the `online` feature and provide your AoC session cookie.

1) Set your session cookie (temporary in current shell):
```bash
export AOC_SESSION="<your-session-cookie>"
```
Or create a `.env` file at the project root containing:
```env
AOC_SESSION=<your-session-cookie>
```
If you use `direnv`, there is a `.envrc` checked in already; you can add `export AOC_SESSION=...` there.

2) Fetch your puzzle input and cache it to `inputs/day01.txt`:
```bash
cargo run --features online -- fetch 1
```

3) Run and submit your answer:
```bash
# Run normally (uses local cached input)
cargo run -- run 1 p1

# Run and submit the computed answer for part 1
cargo run --features online -- run 1 p1 --submit
```

---

# Commands

```bash
# Print path to the input file for a day
cargo run -- input-path 1

# Fetch input (online feature required)
cargo run --features online -- fetch 1            # fails if file exists
cargo run --features online -- fetch 1 --force    # overwrite existing file

# Run a day (offline works if input file exists)
cargo run -- run 1
cargo run -- run 1 p1
cargo run -- run 1 p2

# Submit an answer (online feature required)
cargo run --features online -- run 1 p1 --submit
```

---

# Add a new day solution

1) Create a day module, e.g. `src/day01.rs`:
```rust
use crate::core::Solution;
use color_eyre::Result;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String> {
        // parse and solve
        Ok(input.lines().count().to_string())
    }
    fn part2(&self, _input: &str) -> Result<String> {
        // TODO
        Ok("todo".into())
    }
}
```

2) Register it in `src/core.rs`:
```rust
mod day01; // add at top
static DAY01: day01::Day01 = day01::Day01;

pub fn solution_for(day: u8) -> Option<&'static dyn Solution> {
    match day {
        1 => Some(&DAY01),
        _ => None,
    }
}
```

3) Put your input at `inputs/day01.txt` or run the fetch command:
```bash
cargo run --features online -- fetch 1
```

---

### Project layout
- `src/core.rs` — common utilities, `Solution` trait, input helpers, and optional online fetch/submit.
- `src/cli.rs` — CLI parsing and command dispatch.
- `inputs/` — cached puzzle inputs as `dayXX.txt` (created on first fetch).

### Useful crates (optional)
Common picks for AoC parsing/algorithms (add as needed):
- `itertools`, `regex` or `aho-corasick`, `nom`, `rayon`, `pathfinding`, `ndarray`, `hashbrown`.
We keep dependencies minimal by default; add what you need per-day.

### Development

```bash
# Build
cargo build

# Run unit tests
cargo test
```

Notes:
- Network calls are compiled only with `--features online`.
- Errors show with nice reports via `color-eyre`.
