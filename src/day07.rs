use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    Ok(("".to_string(), "".to_string()))
    // if 3 and one pair: FULL HOUSE
    // if 3 and high card: THREE OF A KIND
    // if two and three of a kind: FULL HOUSE
    // if two and one pair: TWO PAIR
    // if two and no pair: ONE PAIR
}
