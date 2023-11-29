use clap::Parser;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long)]
    year: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    init()?;

    if let Some(day) = cli.day {
        output_result(day, solve(day));
    } else {
        (1..=25).for_each(|i| output_result(i, solve(i)));
    }

    Ok(())
}

fn output_result(day: u8, result: Result<(String, String), Box<dyn Error>>) {
    match result {
        Ok((part1, part2)) => {
            println!("Day {day} Part 1: {part1}");
            println!("Day {day} Part 2: {part2}");
        }
        Err(err) => {
            println!("Error running day {day}: {}", err.to_string());
        }
    }
}

fn read_input(path: PathBuf) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    f.lines()
        .into_iter()
        .collect::<Result<Vec<String>, std::io::Error>>()
}

fn solve(day: u8) -> Result<(String, String), Box<dyn Error>> {
    let input = read_input(PathBuf::from(format!("input-{day}.txt")))?;

    match day {
        1 => day01::solve(input),
        _ => Err(Box::from(format!("Day {day} not implemented"))),
    }
}

fn init() -> Result<(), Box<dyn Error>> {
    let home_path = std::env::var("USERPROFILE")?;
    let aoc_dir = format!("{home_path}/.aoc");
    if !std::path::Path::new(&aoc_dir).is_dir() {
        std::fs::create_dir_all(&aoc_dir)?;
    }

    let session_file_path = format!("{aoc_dir}/session");
    if !std::path::Path::new(&session_file_path).is_file() {
        // throw error
    }

    let mut session_file = fs::File::open(session_file_path)?;
    let mut session_token = String::new();
    session_file.read_to_string(&mut session_token)?;

    let input_1_response = ureq::get("https://adventofcode.com/2022/day/1/input")
        .set("Cookie", &session_token)
        .call()?;

    fs::create_dir_all(format!("{aoc_dir}/2022"))?;

    fs::write(
        format!("{aoc_dir}/2022/input1"),
        input_1_response.into_string()?,
    )?;

    Ok(())
}
