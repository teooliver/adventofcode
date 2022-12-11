// Shapes
// A => Rock  (1)
// B => Paper  (2)
// C => Scissors  (3)
//
// X => Rock  (1)
// Y => Paper  (2)
// Z => Scissors  (3)

// Outcome
// Lost => 0
// Draw => 3
// Won => 6

// Score => Shape + Outcome

use std::{str::FromStr, cmp::Ordering};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock
            && other == &Move::Scissors
        {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

pub fn part_a(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => {
                    3 + moves[1] as u32
                }
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => {
                    0 + moves[1] as u32
                }
                None => {
                    panic!("moves should be comparable")
                }
            }
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::part_a;
    #[test]
    fn test_day_2() {
        assert_eq!(part_a(include_str!("day2_input.txt")), "12855");

    }
}
