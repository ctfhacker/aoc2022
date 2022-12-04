/// The input puzzle
const INPUT: &str = include_str!("../input");

/// The errors that can be triggered during Day 04
#[derive(Debug)]
enum Day04Error {
    InvalidLineFormat(&'static str),
    ParseIntError(std::num::ParseIntError),
}

fn main() -> Result<(), Day04Error> {
    let mut part1_count = 0;
    let mut part2_count = 0;

    // Each line has the form:
    // 1-22,333-44
    //
    // Split each line into the pairs of numbers and check for overlapping or fully
    // contained pairs
    for line in INPUT.lines() {
        let (left, right) = line
            .split_once(',')
            .ok_or(Day04Error::InvalidLineFormat(line))?;

        let (left_min, left_max) = left
            .split_once('-')
            .ok_or(Day04Error::InvalidLineFormat(line))?;

        let left_min = left_min.parse::<u32>().map_err(Day04Error::ParseIntError)?;

        let left_max = left_max.parse::<u32>().map_err(Day04Error::ParseIntError)?;

        let (right_min, right_max) = right
            .split_once('-')
            .ok_or(Day04Error::InvalidLineFormat(line))?;

        let right_min = right_min
            .parse::<u32>()
            .map_err(Day04Error::ParseIntError)?;

        let right_max = right_max
            .parse::<u32>()
            .map_err(Day04Error::ParseIntError)?;

        // Count the number of pairs where one pair contains the other
        if (left_min <= right_min && left_max >= right_max)
            || (right_min <= left_min && right_max >= left_max)
        {
            part1_count += 1;
        }

        // Count the number of pairs where the pairs overlap
        if (left_min <= right_min && left_max >= right_min)
            || (right_min <= left_min && right_max >= left_min)
        {
            part2_count += 1;
        }
    }

    println!("Part 1 count: {part1_count}");
    println!("Part 2 count: {part2_count}");

    Ok(())
}
