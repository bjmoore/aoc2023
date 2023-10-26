use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let body = reqwest::blocking::get("https://adventofcode.com/")?.text()?;

    println!("aoc body = {:?}", body);

    Ok(())
}
