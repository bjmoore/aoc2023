use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut spans: Vec<(u32, bool)> = Vec::new();
    let mut span_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut symbols: Vec<(usize, usize, char)> = Vec::new();

    for (y, line) in input.iter().enumerate() {
        let mut in_digit: bool = false;
        let mut number_start_ptr: usize = 0;
        let mut number_end_ptr: usize = 0;
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if !in_digit {
                    number_start_ptr = x;
                    in_digit = true;
                }
                number_end_ptr = x;
            } else {
                if in_digit {
                    let value: u32 = line[number_start_ptr..number_end_ptr + 1].parse().unwrap();
                    spans.push((value, false));
                    for i in number_start_ptr..number_end_ptr + 1 {
                        span_map.insert((i, y), spans.len() - 1);
                    }
                    in_digit = false;
                }

                if c != '.' {
                    symbols.push((x, y, c));
                }
            }
        }

        if in_digit {
            let value: u32 = line[number_start_ptr..number_end_ptr + 1].parse().unwrap();
            spans.push((value, false));
            for i in number_start_ptr..number_end_ptr + 1 {
                span_map.insert((i, y), spans.len() - 1);
            }
        }
    }

    for &(x, y, _) in symbols.iter() {
        match span_map.get(&(x - 1, y - 1)) {
            Some(index) => spans.get_mut(*index).unwrap().1 = true,
            None => (),
        }
        match span_map.get(&(x, y - 1)) {
            Some(index) => spans.get_mut(*index).unwrap().1 = true,
            None => (),
        }
        match span_map.get(&(x + 1, y - 1)) {
            Some(index) => spans.get_mut(*index).unwrap().1 = true,
            None => (),
        }

        match span_map.get(&(x - 1, y)) {
            Some(index) => spans.get_mut(*index).unwrap().1 = true,
            None => (),
        }
        match span_map.get(&(x + 1, y)) {
            Some(index) => spans.get_mut(*index).unwrap().1 = true,
            None => (),
        }

        match span_map.get(&(x - 1, y + 1)) {
            Some(index) => spans.get_mut(*index).unwrap().1 = true,
            None => (),
        }
        match span_map.get(&(x, y + 1)) {
            Some(index) => spans.get_mut(*index).unwrap().1 = true,
            None => (),
        }
        match span_map.get(&(x + 1, y + 1)) {
            Some(index) => spans.get_mut(*index).unwrap().1 = true,
            None => (),
        }
    }

    let mut adjacent_spans = HashSet::<usize>::new();
    let mut part2: u32 = 0;
    for &(x, y, c) in symbols.iter() {
        if c == '*' {
            match span_map.get(&(x - 1, y - 1)) {
                Some(&index) => adjacent_spans.insert(index),
                None => false,
            };
            match span_map.get(&(x, y - 1)) {
                Some(&index) => adjacent_spans.insert(index),
                None => false,
            };
            match span_map.get(&(x + 1, y - 1)) {
                Some(&index) => adjacent_spans.insert(index),
                None => false,
            };

            match span_map.get(&(x - 1, y)) {
                Some(&index) => adjacent_spans.insert(index),
                None => false,
            };
            match span_map.get(&(x + 1, y)) {
                Some(&index) => adjacent_spans.insert(index),
                None => false,
            };

            match span_map.get(&(x - 1, y + 1)) {
                Some(&index) => adjacent_spans.insert(index),
                None => false,
            };
            match span_map.get(&(x, y + 1)) {
                Some(&index) => adjacent_spans.insert(index),
                None => false,
            };
            match span_map.get(&(x + 1, y + 1)) {
                Some(&index) => adjacent_spans.insert(index),
                None => false,
            };
            if adjacent_spans.len() == 2 {
                part2 += adjacent_spans
                    .iter()
                    .map(|&index| spans.get(index).unwrap().0)
                    .product::<u32>();
            }
        }
        adjacent_spans.clear();
    }

    let part1: u32 = spans
        .iter()
        .filter_map(|(value, touched)| match touched {
            true => Some(value),
            false => None,
        })
        .sum();
    Ok((part1.to_string(), part2.to_string()))
}
