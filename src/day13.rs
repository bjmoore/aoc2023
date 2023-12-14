use std::error::Error;

fn find_mirror(margin: &[u32], start_line: usize) -> Option<usize> {
    let mut palindrome_length = 0;
    for i in 1..margin.len() {
        if i - (palindrome_length * 2) == 0 {
            return Some(i - palindrome_length);
        }
        if margin[i - (1 + palindrome_length * 2)] == margin[i] {
            palindrome_length += 1;
        } else if margin[i - 1] == margin[i] {
            // We need this check to handle a case like this:
            // [45, 82, 82, 82, 82, 45, 12, 8, 12, 82, 97]
            // where there are so many repeating values that it starts trying to match too early.
            palindrome_length = 1;
        } else {
            palindrome_length = 0;
        }
    }

    if palindrome_length > 0 {
        Some(margin.len() - palindrome_length)
    } else {
        None
    }
}

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut part1 = 0;
    let mut start_line = 0;
    let mut end_line = 0;
    for line in &input {
        if line.is_empty() || end_line == input.len() - 1 {
            let height = end_line - start_line;
            let width = input[start_line].len();

            let mut row_margins = vec![0u32; height];
            let mut col_margins = vec![0u32; width];
            for i in 0..height {
                for (j, character) in input[start_line + i].chars().enumerate() {
                    if character == '#' {
                        row_margins[i] += 2u32.pow(j as u32);
                        col_margins[j] += 2u32.pow(i as u32);
                    }
                }
            }

            if let Some(mirror) = find_mirror(&col_margins, start_line) {
                part1 += mirror;
            }
            if let Some(mirror) = find_mirror(&row_margins, 0) {
                part1 += 100 * mirror;
            }

            end_line += 1;
            start_line = end_line;
        } else {
            end_line += 1;
        }
    }

    Ok((part1.to_string(), "".to_string()))
}
