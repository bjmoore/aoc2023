use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let seeds: Vec<u32> = input
        .get(0)
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    Ok(("".to_string(), "".to_string()))
}
