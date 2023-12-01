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
            State::One2 => match c {
                'n' => State::One3,
                _ => State::Init,
            },
            State::InitT => match c {
                'w' => State::Two2,
                'h' => State::Three2,
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
            State::Two2 => Some(2),
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

//one
//two
//three
//four
//five
//six
//seven
//eight
//nine

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut part1 = 0;
    for line in input {
        let mut first_digit = 0;
        let mut last_digit = 0;
        for char in line.chars() {
            if char.is_ascii_digit() {
                if first_digit == 0 {
                    first_digit = char.to_digit(10).unwrap();
                }
                last_digit = char.to_digit(10).unwrap();
            }
        }
        part1 += first_digit * 10 + last_digit;
    }
    Ok((part1.to_string(), "".to_string()))
}
