use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut possible_games: [bool; 100] = [true; 100];

    for (i, line) in input.iter().enumerate() {
        let colon_loc = line.find(':').unwrap();

        let pulls: Vec<&str> = line[colon_loc + 1..].split(';').collect();
        let mut min_rgb: [u32; 3] = [0; 3];
        for pull in pulls {
            let colors: Vec<&str> = pull.split(',').collect();
            let mut rgb: [u32; 3] = [0; 3];
            for color in colors {
                let color = color.trim();
                let parts: Vec<&str> = color.split(' ').collect();
                match parts.get(1).unwrap() {
                    &"red" => {
                        rgb[0] = parts.get(0).unwrap().parse().unwrap();
                    }
                    &"green" => {
                        rgb[1] = parts.get(0).unwrap().parse().unwrap();
                    }
                    &"blue" => {
                        rgb[2] = parts.get(0).unwrap().parse().unwrap();
                    }
                    _ => (),
                };
            }
            if rgb[0] > 12 || rgb[1] > 13 || rgb[2] > 14 {
                possible_games[i] = false;
            }
            if rgb[0] > min_rgb[0] {
                min_rgb[0] = rgb[0];
            }
            if rgb[1] > min_rgb[1] {
                min_rgb[1] = rgb[1];
            }
            if rgb[2] > min_rgb[2] {
                min_rgb[2] = rgb[2];
            }
        }
        part2 += min_rgb[0] * min_rgb[1] * min_rgb[2];
    }

    part1 = possible_games
        .iter()
        .enumerate()
        .filter(|(_, &b)| b)
        .map(|(i, _)| i + 1)
        .sum();
    Ok((part1.to_string(), part2.to_string()))
}
