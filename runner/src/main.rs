use std::path::{Path, PathBuf};

use aoc_2025::AOCYear2025;
use aoc_lib::year::AOCYear;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;

#[derive(Parser)]
#[command(name = "aoc", about = "Advent of Code runner")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(short, long)]
        year: u32,

        #[arg(short, long)]
        day: Option<u8>,
    },

    Download {
        #[arg(short, long)]
        year: u32,

        #[arg(short, long)]
        session: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Run { year, day } => run(year, day),
        Command::Download { year, session } => download(year, &session),
    }
}

fn run(year: u32, day: Option<u8>) {
    let aoc_year: &dyn AOCYear = match year {
        2025 => &AOCYear2025,
        y => panic!("Year {y} is not implemented"),
    };

    if let Some(day) = day {
        if day > aoc_year.num_days() {
            eprintln!(
                "Day {day} is out of range for year {year} (max: {})",
                aoc_year.num_days()
            );
            std::process::exit(1);
        }
        let input = read_input(year, day);
        aoc_year.run_day(day, &input);
    } else {
        for day in 1..=aoc_year.num_days() {
            let input = read_input(year, day);
            aoc_year.run_day(day, &input);
        }
    }
}

fn read_input(year: u32, day: u8) -> String {
    let path = input_path(year, day);
    let mut input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Could not read input file '{}': {e}", path.display()));
    let trimmed_len = input.trim_end_matches('\n').len();
    input.truncate(trimmed_len);
    input
}

fn download(year: u32, session: &str) {
    let num_days: u8 = match year {
        2025 => AOCYear2025.num_days(),
        y => {
            eprintln!("Year {y} is not implemented");
            std::process::exit(1);
        }
    };

    let client = Client::new();
    std::fs::create_dir_all(input_dir(year)).expect("Failed to create input directory");

    for day in 1..=num_days {
        let path = input_path(year, day);
        if path.exists() {
            println!("Day {day:02}: already exists, skipping");
            continue;
        }
        download_day(&client, session, year, day, &path);
    }
}

fn download_day(client: &Client, session: &str, year: u32, day: u8, path: &Path) {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let res = client
        .get(&url)
        .header("Cookie", format!("session={session}"))
        .header(
            "User-Agent",
            "github.com/yourname/aoc-runner by you@example.com",
        )
        .send();

    match res {
        Ok(r) if r.status().is_success() => {
            let text = r.text().expect("Failed to read response body");
            std::fs::write(path, text).expect("Failed to write input file");
            println!("Day {day:02}: downloaded");
        }
        Ok(r) => eprintln!("Day {day:02}: failed with status {}", r.status()),
        Err(e) => eprintln!("Day {day:02}: request error — {e}"),
    }
}

fn input_dir(year: u32) -> PathBuf {
    PathBuf::from("inputs").join(year.to_string())
}

fn input_path(year: u32, day: u8) -> PathBuf {
    input_dir(year).join(format!("day{day:02}.txt"))
}
