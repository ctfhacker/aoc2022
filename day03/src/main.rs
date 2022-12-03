#![deny(clippy::pedantic)]
#![feature(iter_next_chunk)]

/// Puzzle input
const INPUT: &str = include_str!("../input");

/// Possible errors that could happen during Day 03
#[derive(Debug)]
enum Day03Error {
    /// An input line has an odd number of characters
    OddLengthInput(&'static str),

    /// Invalid input chunk found during Part 2
    InvalidChunk(&'static str),

    /// Attempted to process the index for an invalid letter
    InvalidLetterIndex(u8),

    /// Multiple duplicate letters found in this chunk
    MultipleDuplicateLettersFoundInLine(&'static str, &'static str),

    /// Multiple duplicate letters found in this chunk
    MultipleDuplicateLettersFoundInChunk(&'static str, &'static str, &'static str),

    /// No duplicate letters found in this line
    NoDuplicateLettersFoundInLine(&'static str, &'static str),

    /// No duplicate letters found in this chunk
    NoDuplicateLettersFoundInChunk(&'static str, &'static str, &'static str),
}

/// Return the score of the given `byte` based on the following criteria from the puzzle:
/// - Lowercase item types `a` through `z` have priorities  1 through 26.
/// - Uppercase item types `A` through `Z` have priorities 27 through 52.
fn letter_score(value: u8) -> Result<usize, Day03Error> {
    // Calculate the score for lowercase: a = 1, b = 2, ect
    match value {
        b'a'..=b'z' => Ok((value - b'a' + 1) as usize),
        b'A'..=b'Z' => Ok((value - b'A' + 1 + 26) as usize),
        _ => Err(Day03Error::InvalidLetterIndex(value)),
    }
}

fn part1() -> Result<usize, Day03Error> {
    // Initialize the bytes used to calculate the letters seen in the first half of an
    // input line
    let mut seen;

    let mut total_score = 0usize;

    for line in INPUT.lines() {
        seen = [false; 0xff];

        // Sanity check each line is an even length so that it can be split evenly
        if line.len() % 2 == 1 {
            return Err(Day03Error::OddLengthInput(line));
        }

        // Split the input at the mid point
        let (left, right) = line.split_at(line.len() / 2);

        // Use .bytes here since we know the input is a UTF8 string and not unicode
        // Set each letter in the left side as seen
        for letter in left.bytes() {
            seen[letter as usize] = true;
        }

        let mut duplicate_letter = None;

        // For each letter in the right side, check if it has already been seen on the
        // left side. Also, sanity check that there is only one duplicate letter in the
        // right side and not multiple duplicate letters
        for letter in right.bytes() {
            if seen[letter as usize] {
                if let Some(already_found) = duplicate_letter {
                    if already_found != letter {
                        return Err(Day03Error::MultipleDuplicateLettersFoundInLine(left, right));
                    }
                }

                // Set the found duplicate letter
                duplicate_letter = Some(letter);
            }
        }

        // Add the found score to the total score
        if let Some(letter) = duplicate_letter {
            let letter_score = letter_score(letter)?;
            total_score += letter_score;
        } else {
            return Err(Day03Error::NoDuplicateLettersFoundInLine(left, right));
        }
    }

    Ok(total_score)
}

fn part2() -> Result<usize, Day03Error> {
    let mut total_score = 0;

    // Start the input pointer at the beginning of the puzzle input
    let mut input = INPUT;

    // Split the chunk in sets of 3 plus the remaining string
    while !input.is_empty() {
        let mut seen = [0u8; 53];

        // Chunk the input string into 3 lines and keeping the pointer to the rest of the
        // input
        let [first, second, third, rest] = input
            .splitn(4, '\n')
            .next_chunk()
            .map_err(|_| Day03Error::InvalidChunk(input))?;

        // Move the input pointer after the current three lines
        input = rest;

        // Use a bitmask for setting each found letter for each line
        // (line 1, bit 0; line 2, bit 1; line 3, bit 2).
        // Once all three lines have been processed, any letter with the value of 0b111
        // (7) will have been seen by all three lines
        for (index, line) in [first, second, third].iter().enumerate() {
            for letter in line.bytes() {
                let letter_score = letter_score(letter)?;
                seen[letter_score] |= 1 << index;
            }
        }

        // Check for the single 7 value (0b111) value in the seen letters
        let mut found = None;
        for (index, value) in seen.iter().enumerate() {
            if *value == 0b111 {
                if found.is_some() {
                    return Err(Day03Error::MultipleDuplicateLettersFoundInChunk(
                        first, second, third,
                    ));
                }
                found = Some(index);
            }
        }

        // Add the found score to the total score
        if let Some(score) = found {
            total_score += score;
        } else {
            return Err(Day03Error::NoDuplicateLettersFoundInChunk(
                first, second, third,
            ));
        }
    }

    // Return the score
    Ok(total_score)
}

fn main() -> Result<(), Day03Error> {
    let part1_score = part1()?;
    println!("Day 03 Part 1 {part1_score}");

    let part2_score = part2()?;
    println!("Day 03 Part 2 {part2_score}");

    Ok(())
}
