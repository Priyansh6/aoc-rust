# Advent of Code — Rust

Solutions to [Advent of Code](https://adventofcode.com/) puzzles written in Rust, with an emphasis on clean, performant
implementations. Solutions are not hyper-optimised, but care has been taken to choose sensible algorithms and data
structures rather than brute-forcing inputs.

A key part of this project is a **custom parser combinator library** built from scratch in `aoc-lib`. Every solution
declares its input format using composable combinators (`split`, `lines`, `chars`, `map`, `and_then`, wrapped
delimiters, …). This makes parsing concise, type-safe, and consistent across all days.

The project is also designed to be hyper-extensible across years. Adding a new year is a matter of creating a new
crate (e.g. `aoc-2026`), implementing the `AOCYear` trait, and registering it in the runner.

## Features

### Architecture

- **Workspace layout**: three crates: `aoc-lib` (shared utilities), `aoc-2025` (solutions), and `runner` (CLI). New
  years are added as additional solution crates.
- **Trait-based solution interface**: each day implements a `Solution` trait with a `parser`, `part1`, and `part2`.
- **Automatic input download**: the runner can fetch puzzle inputs directly from the AoC website given a session cookie.

### `aoc-lib` - Shared Utilities

A library of reusable primitives that eliminates boilerplate across solutions. It includes a custom parser combinator
framework, generic grid and graph types, geometric algorithms, linear algebra utilities, and various other data
structures that recur across puzzles.

### Performance

A few highlights of non-trivial algorithmic choices:

- **2025 Day 8** — closest-pair algorithm for 3D points with incremental Union-Find to find the answer on the fly
  rather than exhaustive search.
- **2025 Day 10** — Gaussian elimination over GF(2) to prune the button-press search space, combined with memoised
  recursion.
- **2025 Day 12** — bitmask-encoded shape placement with backtracking and a waste budget to prune dead branches early.

## Project Structure

```
aoc-rust/
├── aoc-lib/          # Shared utilities (parser, grid, graph, …)
├── aoc-2025/         # 2025 puzzle solutions
│   └── src/
│       ├── day01.rs … day12.rs
│       └── lib.rs
├── runner/           # CLI entry point
│   └── src/main.rs
└── inputs/           # Puzzle inputs (not committed)
    └── 2025/
        ├── day01.txt … day12.txt
```

## Build

**Prerequisites:** Rust nightly (managed automatically via `rust-toolchain.toml`).

```bash
cargo build --release
```

## Run

### Run a specific day

```bash
cargo run --release -- run --year 2025 --day 1
```

### Run all days for a year

```bash
cargo run --release -- run --year 2025
```

### Download puzzle inputs

Requires an AoC session cookie (found in your browser's cookies after logging in).

```bash
cargo run --release -- download --year 2025 --session <your_session_cookie>
```

Inputs are saved to `inputs/<year>/day<NN>.txt` and skipped if they already exist.

## Adding a New Solution

1. Create `aoc-2025/src/dayNN.rs` implementing the `Solution` trait:

```rust
use aoc_lib::solution::Solution;
use aoc_lib::parser::Parser;

pub struct Sol;

impl Solution for Sol {
    type Parsed = /* your parsed type */;

    fn parser(&self) -> impl Parser<&str, Output=Self::Parsed> {
        todo!()
    }

    fn part1(&self, parsed: &Self::Parsed) -> Option<String> {
        Some(todo!())
    }

    // part2 defaults to None (not implemented) if omitted
}
```

2. Register the module and day in `aoc-2025/src/lib.rs`.
