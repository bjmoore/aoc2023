use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    // at each char: check if we are forced to assign any runs to the current prefix
    // what about a case like this?: ??????..## 1,2
    // what about a case like this?: ?????.#.## 1,2
    // do we need a pass to potentially assign to undefined blocks, and then another pass to fix up ones we were forced to assign later?
    Ok(("".to_string(), "".to_string()))
}
