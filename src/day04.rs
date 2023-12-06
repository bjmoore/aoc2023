use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut part1 = 0;
    let mut card_counts: [u32; 198] = [1; 198];
    for (card_index, line) in input.iter().enumerate() {
        let mut winning_numbers_array: [bool; 100] = [false; 100];
        let (winning_numbers, revealed_numbers) = line.split_once('|').unwrap();
        winning_numbers
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .for_each(|d| winning_numbers_array[d] = true);

        let matches = revealed_numbers
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .filter(|&d| winning_numbers_array[d])
            .count();

        if matches > 0 {
            let mut acc = 1;
            for _ in 0..matches - 1 {
                acc *= 2;
            }
            part1 += acc;

            let current_multiplier = card_counts[card_index];
            for i in (card_index + 1)..(card_index + 1 + matches) {
                card_counts[i] += current_multiplier;
            }
        }
    }

    let part2: u32 = card_counts.iter().sum();
    Ok((part1.to_string(), part2.to_string()))
}
