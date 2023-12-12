use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut seeds: Vec<(u64, bool)> = input
        .get(0)
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|x| (x, false))
        .collect();

    for line in input.iter().skip(2) {
        if line.contains("map") {
            seeds = seeds.iter().map(|&(x, _)| (x, false)).collect();
        }

        if line.starts_with(|c: char| c.is_ascii_digit()) {
            let mut indices = line.match_indices(' ');
            let mid_1 = indices.next().unwrap().0;
            let mid_2 = indices.next().unwrap().0;

            let span = (
                line[0..mid_1].parse::<u64>().unwrap(),
                line[mid_1 + 1..mid_2].parse::<u64>().unwrap(),
                line[mid_2 + 1..].parse::<u64>().unwrap(),
            );

            seeds = seeds
                .iter()
                .map(|&(x, touched)| {
                    if !touched && x >= span.1 && x < span.1 + span.2 {
                        (x + span.0 - span.1, true)
                    } else {
                        (x, touched)
                    }
                })
                .collect();
        }
    }

    println!("{:?}", seeds);

    let part1 = seeds.iter().min_by_key(|(x, _)| x).unwrap().0;
    Ok((part1.to_string(), "".to_string()))
}
