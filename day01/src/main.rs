/// Solve the Day01 Part1 puzzle
///
/// This list represents the Calories of the food carried by five Elves:
///     The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
///     The second Elf is carrying one food item with 4000 Calories.
///     The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
///     The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
///     The fifth Elf is carrying one food item with 10000 Calories.
///
/// In case the Elves get hungry and need extra snacks, they need to know which Elf to
/// ask: they'd like to know how many Calories are being carried by the Elf carrying the
/// most Calories. In the example above, this is 24000 (carried by the fourth Elf).
///
/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn part1(input: &str) {
    let mut max_calories = 0;
    let mut curr_calories = 0;

    // Iterate over all the line
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(num) => {
                // Successful parsing of the line, add it to the current calorie
                // accumulator
                curr_calories += num;
            }
            Err(std::num::ParseIntError { .. }) => {
                if curr_calories > max_calories {
                    max_calories = curr_calories;
                }

                // Always reset the calories after an empty line
                curr_calories = 0;
            }
        }
    }

    // Print the most calories found
    println!("Part 1 Calories: {max_calories}");
}

/// Solve the Day01 Part2 puzzle
///
/// In the example above, the top three Elves are the fourth Elf (with 24000 Calories),
/// then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories).
/// The sum of the Calories carried by these three elves is 45000.

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
fn part2(input: &str) {
    // Keep an array of the current top three calorie counts
    let mut max_calories = [0; 3];
    let mut curr_calories = 0;

    // Iterate over all the line
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(num) => {
                // Successful parsing of the line, add it to the current calorie
                // accumulator
                curr_calories += num;
            }
            Err(std::num::ParseIntError { .. }) => {
                // Hit an empty line, check if the current accumulation is more than any
                // previously seen. If so, save it and break from the loop.
                for max_calorie in max_calories.iter_mut() {
                    if curr_calories > *max_calorie {
                        *max_calorie = curr_calories;
                        break;
                    }
                }

                // Always reset the calories after an empty line
                curr_calories = 0;
            }
        }
    }

    // Print the most calories found
    println!("Part 2 Calories: {}", max_calories.iter().sum::<u32>());
}

fn main() {
    // Include the test case
    let input = include_str!("../input");

    part1(input);
    part2(input);
}
