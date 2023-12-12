use std::collections::HashSet;
use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut occupied_rows: HashSet<usize> = HashSet::new();
    let mut occupied_cols: HashSet<usize> = HashSet::new();
    let mut star_positions: Vec<(usize, usize)> = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.char_indices() {
            if c == '#' {
                occupied_cols.insert(x);
                occupied_rows.insert(y);
                star_positions.push((x, y));
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for (i, &(x, y)) in star_positions.iter().enumerate() {
        for &(k, j) in star_positions.iter().skip(i + 1) {
            // |x - k| + |y - j| + sum(rows and cols in between that are *not* occupied)
            part1 += x.abs_diff(k);
            part1 += y.abs_diff(j);
            part2 += x.abs_diff(k);
            part2 += y.abs_diff(j);
            if k > x {
                for q in x..k {
                    if !occupied_cols.contains(&q) {
                        part1 += 1;
                        part2 += 999999;
                    }
                }
            } else if x > k {
                for q in k..x {
                    if !occupied_cols.contains(&q) {
                        part1 += 1;
                        part2 += 999999;
                    }
                }
            }
            if j > y {
                for q in y..j {
                    if !occupied_rows.contains(&q) {
                        part2 += 999999;
                    }
                }
            } else if y > j {
                for q in j..y {
                    if !occupied_rows.contains(&q) {
                        part1 += 1;
                        part2 += 999999;
                    }
                }
            }
        }
    }

    Ok((part1.to_string(), part2.to_string()))
}
