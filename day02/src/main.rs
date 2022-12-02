//! Day 02 for Advent of Code 2022

#![deny(clippy::pedantic)]
#![deny(missing_docs)]

/// Day02 input problem
const INPUT: &str = include_str!("../input");

/// The errors possible for Day 02
#[derive(Debug, Copy, Clone)]
enum Day02Error {
    /// Attempted to parse an invalid move option
    InvalidMove(char),

    /// Attempted to parse an invalid result option
    InvalidResult(char),

    /// The line length was invalid
    LineLength(usize),
}

/// A particular move in Rock Paper Scissors
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = Day02Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(Day02Error::InvalidMove(value)),
        }
    }
}

/// A particular result condition needed to be satisfied
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NeededResult {
    /// The round must be won by me
    Win,
    /// The round must be lost by me
    Lose,
    /// The round must end in a draw
    Draw,
}

impl TryFrom<char> for NeededResult {
    type Error = Day02Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(NeededResult::Lose),
            'Y' => Ok(NeededResult::Draw),
            'Z' => Ok(NeededResult::Win),
            _ => Err(Day02Error::InvalidResult(value)),
        }
    }
}

fn part1() -> Result<u32, Day02Error> {
    let mut score = 0;

    for line in INPUT.lines() {
        // Sanity check the length of the line is correct
        if line.len() != 3 {
            return Err(Day02Error::LineLength(line.len()));
        }

        // Get the characters from the &str
        let mut chars = line.chars();

        // Expected input `A X`. Parse the values into a `Move` type
        let opponent_choice: Move = chars.next().unwrap().try_into()?;
        let my_choice: Move = chars.nth(1).unwrap().try_into()?;

        // Add the score for the value that I chose
        match my_choice {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scissors => score += 3,
        }

        // Add to the score the outcome of the round
        match (opponent_choice, my_choice) {
            (x, y) if x == y => {
                // Add 3 for a draw
                score += 3;
            }
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => {
                // Add 6 if I win the match
                score += 6;
            }
            _ => {
                // Otherwise, add nothing to the score
            }
        }
    }

    // Return the calculated score
    Ok(score)
}

fn part2() -> Result<u32, Day02Error> {
    let mut score = 0;

    for line in INPUT.lines() {
        // Sanity check the length of the line is correct
        if line.len() != 3 {
            return Err(Day02Error::LineLength(line.len()));
        }

        // Get the characters from the &str
        let mut chars = line.chars();

        // Expected input `A X`. Parse the values into `Move` and `NeededResult` types
        let opponent_choice: Move = chars.next().unwrap().try_into()?;
        let needed_result: NeededResult = chars.nth(1).unwrap().try_into()?;

        let my_choice = match (opponent_choice, needed_result) {
            // Choose the opponents choice for a draw
            (_, NeededResult::Draw) => opponent_choice,

            // Choose the correct response for the opponents choice based on the result
            (Move::Rock, NeededResult::Win) | (Move::Scissors, NeededResult::Lose) => Move::Paper,
            (Move::Rock, NeededResult::Lose) | (Move::Paper, NeededResult::Win) => Move::Scissors,
            (Move::Paper, NeededResult::Lose) | (Move::Scissors, NeededResult::Win) => Move::Rock,
        };

        // Add the score for the value that I chose
        match my_choice {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scissors => score += 3,
        }

        // Add to the score the outcome of the round
        match (opponent_choice, my_choice) {
            (x, y) if x == y => {
                // Add 3 for a draw
                score += 3;
            }
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => {
                // Add 6 if I win the match
                score += 6;
            }
            _ => {
                // Otherwise, add nothing to the score
            }
        }
    }

    // Return the calculated score
    Ok(score)
}

fn main() -> Result<(), Day02Error> {
    let part1_score = part1()?;
    println!("Day02 Part1: {part1_score}");

    let part2_score = part2()?;
    println!("Day02 Part1: {part2_score}");

    Ok(())
}
