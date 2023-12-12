use std::collections::HashMap;
use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut a_nodes: Vec<&str> = Vec::new();
    let directions = input.get(0).unwrap();
    for line in input.iter().skip(2) {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
        if line.chars().nth(2).unwrap() == 'A' {
            a_nodes.push(&line[0..3]);
        }
    }

    let mut location = "AAA";
    let mut part1 = 0;

    for direction in directions.chars().cycle() {
        if location == "ZZZ" {
            break;
        }
        let next_locations = map.get(location).unwrap();
        if direction == 'L' {
            location = next_locations.0;
        } else {
            location = next_locations.1;
        }
        part1 += 1;
    }
    let mut part2 = 0;
    /*

    for (i, direction) in directions.chars().cycle().enumerate() {
        // for each node in a_nodes, iterate until it hits a Z.
        if a_nodes.iter().any(|&n| n.chars().nth(2).unwrap() == 'Z') && i % directions.len() == 0 {
            println!("{a_nodes:?} {part2}");
        }
        a_nodes = a_nodes
            .iter()
            .map(|n| {
                let next_locations = map.get(n).unwrap();
                if direction == 'L' {
                    next_locations.0
                } else {
                    next_locations.1
                }
            })
            .collect();
        part2 += 1;
    }
    */
    Ok((part1.to_string(), part2.to_string()))
}
