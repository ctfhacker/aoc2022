#![feature(iter_next_chunk)]
#![feature(result_option_inspect)]

use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input");

#[derive(Debug, Copy, Clone)]
enum Day09Error {
    /// Invalid direction character
    InvalidDirection(&'static str),

    /// Line failed to parse
    ParseLineFail(&'static str),
}

/// A direction that the rope can move in a 2-dimensional plane
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&'static str> for Direction {
    type Error = Day09Error;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(Day09Error::InvalidDirection(value)),
        }
    }
}

/// A location on a 2-dimensional grid
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    /// Step the location by the given [`Direction`]
    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

/// Execute the rope simulation from Day 09
fn simulation(rope_length: usize) -> Result<usize, Day09Error> {
    // n entries - 1 head and (n-1) tails
    let mut rope = vec![Location::default(); rope_length];

    // Allocate another rope to cache the previous location of each segment
    let mut prev_rope = rope.clone();

    // Initialize the set used to accumulate the locations the tail travels
    let mut tail_steps = BTreeSet::new();
    tail_steps.insert(rope[rope_length - 1]);

    for line in INPUT.split('\n') {
        // Ignore empty lines
        if line.is_empty() {
            continue;
        }

        // Parse each line `U 1` into `Direction::Up 1`
        let Ok((Ok(direction), Ok(number_of_steps))) = line
                .split(' ')
                .next_chunk()
                .map(|[left, right]| (Direction::try_from(left), right.parse::<u32>())) else {
            return Err(Day09Error::ParseLineFail(line));
        };

        for _ in 0..number_of_steps {
            // Simulate the movement of the head
            rope[0].step(direction);

            // Treat each 2 chunk window of the rope as a (head, tail) pair. The tail
            // segment will only move when the head segment is a certain configuration
            // away
            for index in 1..rope.len() {
                let curr_head = rope[index - 1];
                let curr_tail = rope[index];

                // Initialize the new position of the current segment
                let mut new_position: Option<Location> = None;

                // Rope is moving horizontally
                // .....    .....    .....
                // .TH.. -> .T.H. -> ..TH.
                // .....    .....    .....
                if (curr_head.x - curr_tail.x).abs() == 2 && (curr_head.y - curr_tail.y).abs() == 0
                {
                    if curr_head.x > curr_tail.x {
                        new_position = Some(Location {
                            x: curr_head.x - 1,
                            y: curr_head.y,
                        });
                    } else {
                        new_position = Some(Location {
                            x: curr_head.x + 1,
                            y: curr_head.y,
                        });
                    }
                }

                // Rope is moving vertically
                // ...    ...    ...
                // .T.    .T.    ...
                // .H. -> ... -> .T.
                // ...    .H.    .H.
                // ...    ...    ...
                if (curr_head.y - curr_tail.y).abs() == 2 && (curr_head.x - curr_tail.x).abs() == 0
                {
                    if curr_head.y > curr_tail.y {
                        new_position = Some(Location {
                            x: curr_head.x,
                            y: curr_head.y - 1,
                        });
                    } else {
                        new_position = Some(Location {
                            x: curr_head.x,
                            y: curr_head.y + 1,
                        });
                    }
                }

                // Rope has moved into an L shaped pattern vertically
                //  .....    .....    .....
                // .....    ..H..    ..H..
                // ..H.. -> ..... -> ..T..
                // .T...    .T...    .....
                // .....    .....    .....
                if (curr_head.x - curr_tail.x).abs() == 1 && (curr_head.y - curr_tail.y).abs() == 2
                {
                    if curr_head.y > curr_tail.y {
                        new_position = Some(Location {
                            x: curr_head.x,
                            y: curr_head.y - 1,
                        });
                    } else {
                        new_position = Some(Location {
                            x: curr_head.x,
                            y: curr_head.y + 1,
                        });
                    }
                }

                // Rope has moved into an L shaped pattern horizontally
                // .....    .....    .....
                // .....    .....    .....
                // ..H.. -> ...H. -> ..TH.
                // .T...    .T...    .....
                // .....    .....    .....
                if (curr_head.x - curr_tail.x).abs() == 2 && (curr_head.y - curr_tail.y).abs() == 1
                {
                    if curr_head.x > curr_tail.x {
                        new_position = Some(Location {
                            x: curr_head.x - 1,
                            y: curr_head.y,
                        });
                    } else {
                        new_position = Some(Location {
                            x: curr_head.x + 1,
                            y: curr_head.y,
                        });
                    }
                }

                // Rope has moved into a diagonal pattern
                // .....    .....    .....
                // .....    ...H.    ...H.
                // ..H.. -> ..... -> ..T..
                // .T...    .T...    .....
                // .....    .....    .....
                if (curr_head.x - curr_tail.x).abs() == 2 && (curr_head.y - curr_tail.y).abs() == 2
                {
                    if curr_head.x > curr_tail.x && curr_head.y > curr_tail.y {
                        new_position = Some(Location {
                            x: curr_head.x - 1,
                            y: curr_head.y - 1,
                        });
                    } else if curr_head.x > curr_tail.x && curr_head.y < curr_tail.y {
                        new_position = Some(Location {
                            x: curr_head.x - 1,
                            y: curr_head.y + 1,
                        });
                    } else if curr_head.x < curr_tail.x && curr_head.y > curr_tail.y {
                        new_position = Some(Location {
                            x: curr_head.x + 1,
                            y: curr_head.y - 1,
                        });
                    } else if curr_head.x < curr_tail.x && curr_head.y < curr_tail.y {
                        new_position = Some(Location {
                            x: curr_head.x + 1,
                            y: curr_head.y + 1,
                        });
                    }
                }

                // If the (head, tail) pair is ever more than 2 segments away, one of the
                // above simulation movements is wrong. Immediately dump the simualation
                // state and panic
                if (curr_head.x - curr_tail.x).abs() > 2 && (curr_head.y - curr_tail.y).abs() > 2 {
                    println!("{rope:?}");
                    println!("------------------ {index} ----------------");
                    for y in 10..40 {
                        for x in -40..40 {
                            let curr = Location { x, y };
                            /*
                            if curr == Location::default() {
                                print!("s");
                            }
                            */
                            if let Some(index) = rope.iter().position(|&x| x == curr) {
                                if index == 0 {
                                    print!("H");
                                } else if index == 10 {
                                    print!("T");
                                } else {
                                    print!("{index}");
                                }
                            } else {
                                print!(".");
                            }
                        }
                        println!();
                    }

                    panic!("Link is too far! Check the new conditions");
                }

                // If any of the movement conditions are true, set the tail to the previous
                // head position
                if let Some(new_pos) = new_position {
                    // Update this ropes position
                    rope[index] = new_pos;

                    // Solution only cares about the last tail positions
                    if index == rope_length - 1 {
                        tail_steps.insert(rope[index]);
                    }
                }
            }

            // Update the head position
            prev_rope[0] = rope[0];
        }
    }

    // Return the number of steps moved by the tail
    Ok(tail_steps.len())
}

fn main() -> Result<(), Day09Error> {
    let part1 = simulation(2)?;
    println!("Part 1 tailed moved: {part1}");
    let part2 = simulation(10)?;
    println!("Part 2 tailed moved: {part2}");

    Ok(())
}
