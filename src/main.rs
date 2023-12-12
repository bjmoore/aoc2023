use chrono::prelude::*;
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

    #[arg(short, long, default_value = "2023")]
    year: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    init(&cli.year)?;

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
    let home_path = std::env::var("USERPROFILE")?;
    let aoc_dir = format!("{home_path}/.aoc");
    let input = read_input(PathBuf::from(format!("{aoc_dir}/2023/input{day}")))?;

    match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        7 => day07::solve(input),
        8 => day08::solve(input),
        10 => day10::solve(input),
        11 => day11::solve(input),
        _ => Err(Box::from(format!("Day {day} not implemented"))),
    }
}

fn init(year: &str) -> Result<(), Box<dyn Error>> {
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

    let eastern_standard_time = FixedOffset::west_opt(5 * 3600).unwrap();
    let current_time = Utc::now().with_timezone(&eastern_standard_time);
    let aocs_released = current_time.day(); // haha
    println!("Current day: {aocs_released}");

    fs::create_dir_all(format!("{aoc_dir}/{year}"))?;

    for i in 1..=aocs_released {
        let input_file_path = format!("{aoc_dir}/{year}/input{i}");
        if !std::path::Path::new(&input_file_path).is_file() {
            let input_url = format!("https://adventofcode.com/{year}/day/{i}/input");
            let input_response = ureq::get(&input_url).set("Cookie", &session_token).call()?;

            fs::write(input_file_path, input_response.into_string()?)?;
        }
    }

    Ok(())
}
