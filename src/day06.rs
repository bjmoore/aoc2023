use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut part1: u32 = 1;
    let times: Vec<f32> = input
        .get(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<f32>().unwrap())
        .collect();
    let distances: Vec<f32> = input
        .get(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<f32>().unwrap())
        .collect();

    for (&t, d) in times.iter().zip(distances) {
        let discriminant = (t * t - 4_f32 * d).sqrt();
        let hi = (t + discriminant) / 2.0;
        let lo = (t - discriminant) / 2.0;

        let hi = hi.ceil() as u32;
        let lo = lo.ceil() as u32;

        part1 *= hi - lo;
    }

    let big_time = input
        .get(0)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<f32>()
        .unwrap();
    let big_distance = input
        .get(1)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<f32>()
        .unwrap();

    let discriminant = (big_time * big_time - 4_f32 * big_distance).sqrt();
    let hi = (big_time + discriminant) / 2.0;
    let lo = (big_time - discriminant) / 2.0;

    let hi = hi.ceil() as u32;
    let lo = lo.ceil() as u32;

    let part2 = hi - lo;

    Ok((part1.to_string(), part2.to_string()))
}
