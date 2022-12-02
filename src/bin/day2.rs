
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug))]
enum Figure {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Figure {
    fn wins_against(&self) -> Figure {
        match self {
            Figure::Rock => Figure::Scissors,
            Figure::Paper => Figure::Rock,
            Figure::Scissors => Figure::Paper,
        }
    }

    fn loses_against(&self) -> Figure {
        match self {
            Figure::Rock => Figure::Paper,
            Figure::Paper => Figure::Scissors,
            Figure::Scissors => Figure::Rock,
        }
    }
}

impl From<char> for Figure {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Figure::Rock,
            'B' | 'Y' => Figure::Paper,
            'C' | 'Z' => Figure::Scissors,
            _ => panic!("Unknown figure: {}", c)
        }
    }
}

#[cfg_attr(test, derive(Clone, PartialEq, Debug))]
struct Round(Figure, Figure);

impl From<&str> for Round {
    fn from(s: &str) -> Self {
        let (a, b) = (s.chars().nth(0).unwrap(), s.chars().nth(2).unwrap());
        Round(Figure::from(a), Figure::from(b))
    }
}

impl From<&RoundStrategy> for Round {
    fn from(r: &RoundStrategy) -> Self {
        Round(r.0, match r.1 {
            Strategy::Lose => r.0.wins_against(),
            Strategy::Draw => r.0,
            Strategy::Win => r.0.loses_against(),
        })
    }
}

impl Round {
    fn score(self) -> i32 {
        self.1 as i32 + 
            if self.1.wins_against() == self.0 { 6 } 
            else if self.1.loses_against() == self.0 { 0 }
            else { 3 }
    }
}

#[cfg_attr(test, derive(Clone, Copy, Debug, PartialEq))]
enum Strategy {
    Lose,
    Draw,
    Win,
}

impl From<char> for Strategy {
    fn from(c: char) -> Self {
        match c {
            'X' => Strategy::Lose,
            'Y' => Strategy::Draw,
            'Z' => Strategy::Win,
            _ => panic!("Unknown strategy: {}", c)
        }
    }
}

#[cfg_attr(test, derive(Clone, Copy, Debug, PartialEq))]
struct RoundStrategy(Figure, Strategy);

impl From<&str> for RoundStrategy {
    fn from(s: &str) -> Self {
        let (a, b) = (s.chars().nth(0).unwrap(), s.chars().nth(2).unwrap());
        RoundStrategy(Figure::from(a), Strategy::from(b))
    }
}

impl RoundStrategy {
    fn round(self) -> Round {
        Round::from(&self)
    }
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(Round::from)
        .map(Round::score)
        .sum()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(RoundStrategy::from)
        .map(RoundStrategy::round)
        .map(Round::score)
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/2.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(Round(Figure::Rock, Figure::Paper).score(), 8);
        assert_eq!(Round(Figure::Paper, Figure::Rock).score(), 1);
        assert_eq!(Round(Figure::Scissors, Figure::Scissors).score(), 6);
    }

    #[test]
    fn test_char_to_figure() {
        assert_eq!(Figure::from('A'), Figure::Rock);
        assert_eq!(Figure::from('B'), Figure::Paper);
        assert_eq!(Figure::from('C'), Figure::Scissors);

        assert_eq!(Figure::from('X'), Figure::Rock);
        assert_eq!(Figure::from('Y'), Figure::Paper);
        assert_eq!(Figure::from('Z'), Figure::Scissors);
    }

    #[test]
    fn test_figure_wins_against() {
        assert_eq!(Figure::Rock.wins_against(), Figure::Scissors);
        assert_eq!(Figure::Paper.wins_against(), Figure::Rock);
        assert_eq!(Figure::Scissors.wins_against(), Figure::Paper);
    }

    #[test]
    fn test_figure_loses_against() {
        assert_eq!(Figure::Rock.loses_against(), Figure::Paper);
        assert_eq!(Figure::Paper.loses_against(), Figure::Scissors);
        assert_eq!(Figure::Scissors.loses_against(), Figure::Rock);
    }

    #[test]
    fn test_round_strategy_from() {
        assert_eq!(RoundStrategy::from("A X"), RoundStrategy(Figure::Rock, Strategy::Lose));
        assert_eq!(RoundStrategy::from("B Y"), RoundStrategy(Figure::Paper, Strategy::Draw));
        assert_eq!(RoundStrategy::from("C Z"), RoundStrategy(Figure::Scissors, Strategy::Win));
    }

    #[test]
    fn test_strategy_to_round() {
        assert_eq!(Round::from(&RoundStrategy(Figure::Rock, Strategy::Lose)), Round(Figure::Rock, Figure::Scissors));
        assert_eq!(Round::from(&RoundStrategy(Figure::Paper, Strategy::Draw)), Round(Figure::Paper, Figure::Paper));
        assert_eq!(Round::from(&RoundStrategy(Figure::Scissors, Strategy::Win)), Round(Figure::Scissors, Figure::Rock));
    }

    #[test]
    fn test_part_1() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(part_1(input), 15);
    }

    #[test]
    fn test_part_2() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(part_2(input), 12);
    }
}