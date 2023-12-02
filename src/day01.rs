use std::error::Error;

struct StateMachine {
    state: State,
}
#[derive(PartialEq)]
enum State {
    Init,
    One1,
    One2,
    One3,
    InitT,
    Two2,
    Two3,
    Three2,
    Three3,
    Three4,
    Three5,
    InitF,
    Four2,
    Four3,
    Four4,
    Five2,
    Five3,
    Five4,
    InitS,
    Six2,
    Six3,
    Seven2,
    Seven3,
    Seven4,
    Seven5,
    Eight1,
    Eight2,
    Eight3,
    Eight4,
    Eight5,
    Nine1,
    Nine2,
    Nine3,
    Nine4,
}

impl StateMachine {
    fn match_digit(&mut self, c: char) -> Option<u32> {
        self.state = match &self.state {
            State::One1 => match c {
                'n' => State::One2,
                _ => State::Init,
            },
            State::One2 => match c {
                'i' => State::Nine2,
                'e' => State::One3,
                _ => State::Init,
            },
            State::One3 => match c {
                'i' => State::Eight2,
                _ => State::Init,
            },
            State::InitT => match c {
                'w' => State::Two2,
                'h' => State::Three2,
                _ => State::Init,
            },
            State::Two2 => match c {
                'o' => State::Two3,
                _ => State::Init,
            },
            State::Two3 => match c {
                'n' => State::One2,
                _ => State::Init,
            },
            State::Three2 => match c {
                'r' => State::Three3,
                _ => State::Init,
            },
            State::Three3 => match c {
                'e' => State::Three4,
                _ => State::Init,
            },
            State::Three4 => match c {
                'e' => State::Three5,
                'i' => State::Eight2,
                _ => State::Init,
            },
            State::Three5 => match c {
                'i' => State::Eight2,
                _ => State::Init,
            },
            State::InitF => match c {
                'o' => State::Four2,
                'i' => State::Five2,
                _ => State::Init,
            },
            State::Four2 => match c {
                'n' => State::One2,
                'u' => State::Four3,
                _ => State::Init,
            },
            State::Four3 => match c {
                'r' => State::Four4,
                _ => State::Init,
            },
            State::Five2 => match c {
                'v' => State::Five3,
                _ => State::Init,
            },
            State::Five3 => match c {
                'e' => State::Five4,
                _ => State::Init,
            },
            State::InitS => match c {
                'i' => State::Six2,
                'e' => State::Seven2,
                _ => State::Init,
            },
            State::Six2 => match c {
                'x' => State::Six3,
                _ => State::Init,
            },
            State::Seven2 => match c {
                'i' => State::Eight2,
                'v' => State::Seven3,
                _ => State::Init,
            },
            State::Seven3 => match c {
                'e' => State::Seven4,
                _ => State::Init,
            },
            State::Seven4 => match c {
                'i' => State::Eight2,
                'n' => State::Seven5,
                _ => State::Init,
            },
            State::Eight1 => match c {
                'i' => State::Eight2,
                _ => State::Init,
            },
            State::Eight2 => match c {
                'g' => State::Eight3,
                _ => State::Init,
            },
            State::Eight3 => match c {
                'h' => State::Eight4,
                _ => State::Init,
            },
            State::Eight4 => match c {
                't' => State::Eight5,
                _ => State::Init,
            },
            State::Eight5 => match c {
                'w' => State::Two2,
                'h' => State::Three3,
                _ => State::Init,
            },
            State::Nine1 => match c {
                'i' => State::Nine2,
                _ => State::Init,
            },
            State::Nine2 => match c {
                'n' => State::Nine3,
                _ => State::Init,
            },
            State::Nine3 => match c {
                'e' => State::Nine4,
                _ => State::Init,
            },
            State::Nine4 => match c {
                'i' => State::Eight2,
                _ => State::Init,
            },
            _ => State::Init,
        };

        if self.state == State::Init {
            self.state = match c {
                'o' => State::One1,
                't' => State::InitT,
                'f' => State::InitF,
                's' => State::InitS,
                'e' => State::Eight1,
                'n' => State::Nine1,
                _ => State::Init,
            }
        }

        match &self.state {
            State::One3 => Some(1),
            State::Two3 => Some(2),
            State::Three5 => Some(3),
            State::Four4 => Some(4),
            State::Five4 => Some(5),
            State::Six3 => Some(6),
            State::Seven5 => Some(7),
            State::Eight5 => Some(8),
            State::Nine4 => Some(9),
            _ => None,
        }
    }
}

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input {
        let mut first_digit_one = 0;
        let mut last_digit_one = 0;
        let mut first_digit_two = 0;
        let mut last_digit_two = 0;
        let mut state_machine = StateMachine { state: State::Init };
        for char in line.chars() {
            let next_digit_part_one = if char.is_ascii_digit() {
                Some(char.to_digit(10).unwrap())
            } else {
                None
            };

            let next_digit_part_two = state_machine.match_digit(char);

            match next_digit_part_one {
                Some(d) => {
                    if first_digit_one == 0 {
                        first_digit_one = d
                    };
                    if first_digit_two == 0 {
                        first_digit_two = d
                    };
                    last_digit_one = d;
                    last_digit_two = d;
                }
                None => (),
            }
            match next_digit_part_two {
                Some(d) => {
                    if first_digit_two == 0 {
                        first_digit_two = d
                    };
                    last_digit_two = d;
                }
                None => (),
            }
        }

        // Some code that roughly implements the "naive" solution, which I used for debugging my state machine:
        /*
        let first_digit_validation = words
            .iter()
            .filter_map(|s| {
                if let Some(d) = line.find(s) {
                    Some((d, s))
                } else {
                    None
                }
            })
            .min_by(|x, y| x.0.cmp(&y.0))
            .unwrap_or((0, &""))
            .1;
        let first_digit_validation = match first_digit_validation {
            &"one" => 1,
            &"two" => 2,
            &"three" => 3,
            &"four" => 4,
            &"five" => 5,
            &"six" => 6,
            &"seven" => 7,
            &"eight" => 8,
            &"nine" => 9,
            _ => 0,
        };
        if first_digit_validation != first_digit_two {
            println!("{linect}: {first_digit_two} {first_digit_validation}");
        }
        let last_digit_validation = words
            .iter()
            .filter_map(|s| {
                if let Some(d) = line.rfind(s) {
                    Some((d, s))
                } else {
                    None
                }
            })
            .max_by(|x, y| x.0.cmp(&y.0))
            .unwrap_or((0, &""))
            .1;
        let last_digit_validation = match last_digit_validation {
            &"one" => 1,
            &"two" => 2,
            &"three" => 3,
            &"four" => 4,
            &"five" => 5,
            &"six" => 6,
            &"seven" => 7,
            &"eight" => 8,
            &"nine" => 9,
            _ => 0,
        };
        if last_digit_validation != last_digit_two {
            println!("{linect}: {last_digit_two} {last_digit_validation}");
        }
        */

        part1 += first_digit_one * 10 + last_digit_one;
        part2 += first_digit_two * 10 + last_digit_two;
    }
    Ok((part1.to_string(), part2.to_string()))
}
