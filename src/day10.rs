use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;

fn index(x: usize, y: usize, stride: usize) -> usize {
    x + y * stride
}

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let columns = input.get(0).unwrap().len();
    let rows = input.len();
    let mut flat_map: Vec<u8> = Vec::new();
    let mut start = (0, 0);
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            flat_map.push(match c {
                '|' => 1,
                '-' => 2,
                'L' => 3,
                'J' => 4,
                '7' => 5,
                'F' => 6,
                'S' => 7,
                _ => 0,
            });

            if c == 'S' {
                start = (x, y);
            }
        }
    }

    // 3rd element is current direction- N E W S
    let mut walkers: Vec<(usize, usize, u8)> = Vec::new();

    // find the two pipes connecting to S
    let north = flat_map.get(index(start.0, start.1 - 1, columns)).unwrap();
    let east = flat_map.get(index(start.0 + 1, start.1, columns)).unwrap();
    let west = flat_map.get(index(start.0 - 1, start.1, columns)).unwrap();
    let south = flat_map.get(index(start.0, start.1 + 1, columns)).unwrap();

    if *north == 1 || *north == 5 || *north == 6 {
        walkers.push((start.0, start.1 - 1, 1));
    }
    if *east == 2 || *east == 4 || *east == 5 {
        walkers.push((start.0 + 1, start.1, 2));
    }
    if *west == 2 || *west == 3 || *west == 6 {
        walkers.push((start.0 - 1, start.1, 3));
    }
    if *south == 1 || *south == 3 || *south == 4 {
        walkers.push((start.0, start.1 + 1, 4));
    }

    // start walking
    let mut part1 = 1;
    let mut loop_boundary: HashSet<(usize, usize)> = HashSet::new();
    loop {
        walkers = walkers
            .iter()
            .map(|&(x, y, dir)| {
                let pipe = flat_map.get(index(x, y, columns)).unwrap();
                match pipe {
                    // |
                    1 => match dir {
                        1 => (x, y - 1, 1),
                        4 => (x, y + 1, 4),
                        _ => (x, y, dir),
                    },
                    // -
                    2 => match dir {
                        2 => (x + 1, y, 2),
                        3 => (x - 1, y, 3),
                        _ => (x, y, dir),
                    },
                    // L
                    3 => match dir {
                        3 => (x, y - 1, 1),
                        4 => (x + 1, y, 2),
                        _ => (x, y, dir),
                    },
                    // J
                    4 => match dir {
                        2 => (x, y - 1, 1),
                        4 => (x - 1, y, 3),
                        _ => (x, y, dir),
                    },
                    // 7
                    5 => match dir {
                        1 => (x - 1, y, 3),
                        2 => (x, y + 1, 4),
                        _ => (x, y, dir),
                    },
                    // F
                    6 => match dir {
                        1 => (x + 1, y, 2),
                        3 => (x, y + 1, 4),
                        _ => (x, y, dir),
                    },
                    _ => (x, y, dir),
                }
            })
            .collect();
        part1 += 1;
        let w1 = walkers.get(0).unwrap();
        let w2 = walkers.get(1).unwrap();
        loop_boundary.insert((w1.0, w2.1));
        loop_boundary.insert((w2.0, w2.1));
        if w1.0 == w2.0 && w1.1 == w2.1 {
            break;
        }
    }

    // start at 0,0 and walk a pointer toward S until we hit the pipe loop
    // result -> single coordinate with orientation bit
    // then we can flood from the *inside* with a handedness bit
    // result -> an integer

    Ok((part1.to_string(), "".to_string()))
}
