use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}


impl Move {
    fn beats(&self, other: &Move) -> bool {
        match (self, other) {
            (Move::Rock, Move::Scissors) => true,
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            _ => false
        }
    }

    fn as_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}

struct Round {
    opponent: Move,
    player: Move
}

impl Round {
    fn from_line(line: &str) -> Round {
        let mut parts = line.split_whitespace();

        let opponent = match parts.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move")
        };

        let player = match parts.next().unwrap() {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid move")
        };
        Round { opponent, player }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss
}

impl Outcome {
    fn from_round(round: Round) -> Outcome {
        if round.opponent.beats(&round.player) {
            Outcome::Loss
        } else if round.player.beats(&round.opponent) {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }

    fn as_score(&self) -> u32 {
        match self {
            Outcome::Win =>  6,
            Outcome::Draw => 3,
            Outcome::Loss => 0
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_score = input
        .lines()
        .map(Round::from_line)
        .map(|r| (r.player, Outcome::from_round(r)))
        .map(|(player, outcome)| player.as_score() + outcome.as_score())
        .sum();


    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
