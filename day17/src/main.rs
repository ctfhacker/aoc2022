#![deny(clippy::pedantic)]
use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("../input");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Piece {
    HorizLine,
    Plus,
    L,
    VertLine,
    Square,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Filled,
}

#[derive(Debug, Copy, Clone)]
enum Movement {
    Left,
    Right,
    Down,
}

/// The width of the grid
const GRID_WIDTH: usize = 7;

const GRID_HEIGHT: usize = 256;

#[allow(clippy::too_many_lines)]
fn main() {
    let pieces = vec![
        Piece::HorizLine,
        Piece::Plus,
        Piece::L,
        Piece::VertLine,
        Piece::Square,
    ];

    // Initialize the cyclic buffer
    let mut grid = vec![Space::Empty; GRID_WIDTH * GRID_HEIGHT];

    // Bottom row is filled
    for space in grid.iter_mut().take(GRID_WIDTH) {
        *space = Space::Filled;
    }

    // Convert the input into `Movement` enum
    let mut movement = INPUT
        .iter()
        .filter_map(|x| {
            if *x == b'<' {
                Some(Movement::Left)
            } else if *x == b'>' {
                Some(Movement::Right)
            } else {
                None
            }
        })
        .cycle();

    let mut curr_height = 0;

    // Known states that have been observed in the simulation. Once a known state has been seen again,
    // we know a cycle has been found and can calculate the remaining height manually
    let mut states = HashMap::new();

    // Storage of the height difference added at each step, used for the height calculation once a
    // cycle has been found
    let mut diffs = Vec::new();

    'next_piece: for (piece_index, curr_piece) in
        pieces.iter().cycle().take(1_000_000_000_000).enumerate()
    {
        // Print the part 1 answer at index 2022
        if piece_index == 2022 {
            println!("Part 1: {curr_height}");
        }

        // Simulate until a cycle is found
        if let Some((old_index, old_height)) =
            states.insert((curr_piece, grid.clone()), (piece_index, curr_height))
        {
            // Once a cycle has been found, we can math the remaining iterations
            let iter_diff = piece_index - old_index;
            let height_diff = curr_height - old_height;
            let remaining = 1_000_000_000_000 - piece_index;
            let cycles = remaining / iter_diff;
            let iters_left = remaining % iter_diff;

            println!("Cycle found at {piece_index}: ({old_index}, {old_height}) -> ({piece_index}, {curr_height})");
            println!("Every {iter_diff} height goes up {height_diff}");
            println!("Remaining: {remaining} Cycles {cycles} Iters left {iters_left}");

            // Do the rest of the math for to calculate the height increases
            curr_height += cycles * height_diff;
            for diff in diffs.iter().skip(old_index).take(iters_left) {
                curr_height += diff;
            }

            println!("Part 2 Curr height: {curr_height}");
            break;
        }

        // println!("Piece: {piece_index}");
        let grid_height = curr_height % GRID_HEIGHT;

        // Clear room to place the piece since the grid is a circular buffer
        for y in (1..8).rev() {
            let row = (grid_height + y) % GRID_HEIGHT;
            grid[row * GRID_WIDTH..(row + 1) * GRID_WIDTH]
                .iter_mut()
                .for_each(|space| *space = Space::Empty);
        }

        // Get the coordinates for the next piece
        #[allow(clippy::identity_op)]
        let mut piece = match curr_piece {
            Piece::HorizLine => {
                // ..====.
                vec![
                    (2, (grid_height + 4) % GRID_HEIGHT),
                    (3, (grid_height + 4) % GRID_HEIGHT),
                    (4, (grid_height + 4) % GRID_HEIGHT),
                    (5, (grid_height + 4) % GRID_HEIGHT),
                ]
            }
            Piece::Plus => {
                // ...=...
                // ..===..
                // ...=...
                vec![
                    (3, ((grid_height + 0 + 4) % GRID_HEIGHT)),
                    (2, ((grid_height + 1 + 4) % GRID_HEIGHT)),
                    (3, ((grid_height + 1 + 4) % GRID_HEIGHT)),
                    (4, ((grid_height + 1 + 4) % GRID_HEIGHT)),
                    (3, ((grid_height + 2 + 4) % GRID_HEIGHT)),
                ]
            }
            Piece::L => {
                // ....=
                // ....=
                // ..===
                vec![
                    (4, ((grid_height + 2 + 4) % GRID_HEIGHT)),
                    (4, ((grid_height + 1 + 4) % GRID_HEIGHT)),
                    (2, ((grid_height + 0 + 4) % GRID_HEIGHT)),
                    (3, ((grid_height + 0 + 4) % GRID_HEIGHT)),
                    (4, ((grid_height + 0 + 4) % GRID_HEIGHT)),
                ]
            }
            Piece::VertLine => {
                // ..=..
                // ..=..
                // ..=..
                // ..=..
                vec![
                    (2, ((grid_height + 0 + 4) % GRID_HEIGHT)),
                    (2, ((grid_height + 1 + 4) % GRID_HEIGHT)),
                    (2, ((grid_height + 2 + 4) % GRID_HEIGHT)),
                    (2, ((grid_height + 3 + 4) % GRID_HEIGHT)),
                ]
            }
            Piece::Square => {
                // ..==
                // ..==
                vec![
                    (2, ((grid_height + 0 + 4) % GRID_HEIGHT)),
                    (3, ((grid_height + 0 + 4) % GRID_HEIGHT)),
                    (2, ((grid_height + 1 + 4) % GRID_HEIGHT)),
                    (3, ((grid_height + 1 + 4) % GRID_HEIGHT)),
                ]
            }
        };

        // Continue moving the current piece until it cannot move down anymore
        for iter in 0.. {
            let curr_move = if iter % 2 == 0 {
                movement.next().unwrap()
            } else {
                Movement::Down
            };

            match curr_move {
                Movement::Left => {
                    // Cannot move the piece left if any piece is on the left border
                    if piece.iter().any(|(x, y)| {
                        let position = y * GRID_WIDTH + (x - 1);
                        *x == 0 || grid[position] == Space::Filled
                    }) {
                        continue;
                    }

                    // Move the piece left
                    piece.iter_mut().for_each(|(x, _y)| *x -= 1);
                }
                Movement::Right => {
                    // Cannot move the piece right if any piece is on the right border
                    if piece.iter().any(|(x, y)| {
                        let position = y * GRID_WIDTH + (x + 1);
                        *x == GRID_WIDTH - 1 || grid[position] == Space::Filled
                    }) {
                        continue;
                    }

                    // Move the piece right
                    piece.iter_mut().for_each(|(x, _y)| *x += 1);
                }
                Movement::Down => {
                    // Cannot move the piece down if any piece is under the current piece
                    if piece.iter().any(|(x, y)| {
                        #[allow(clippy::cast_possible_truncation)]
                        let y = (*y as u8).wrapping_sub(1) as usize;
                        let position = y * GRID_WIDTH + x;
                        grid[position] == Space::Filled
                    }) {
                        // Piece cannot move down any further. Write the piece into the grid.
                        for (x, y) in &piece {
                            let position = y * GRID_WIDTH + x;
                            grid[position] = Space::Filled;
                        }

                        // Normally, take the height of the highest part of the piece
                        let mut h = piece.iter().map(|(_x, y)| *y).max().unwrap();

                        // If a resting piece is currently straddling the cyclic buffer,
                        // Take the highest height under 8
                        if piece.iter().filter(|(_x, y)| *y == 255).count() > 0
                            && piece.iter().filter(|(_x, y)| *y == 0).count() > 0
                        {
                            h = piece
                                .iter()
                                .filter(|(_x, y)| *y < 8)
                                .map(|(_x, y)| *y)
                                .max()
                                .unwrap();
                        }

                        let diff = if h > grid_height {
                            let diff = h - grid_height;

                            // Don't count the blocks that wrap around during falling
                            if diff <= 6 {
                                diff
                            } else {
                                0
                            }
                        } else if h < 8 && grid_height > 225 && curr_height > 200 {
                            // Ensure that the height is accounted for during a block
                            // that is wrapping the cyclic buffer
                            h + GRID_HEIGHT - grid_height
                        } else {
                            0
                        };

                        // Add the current difference to the diffs storage
                        // Iteration X's height diff will be diffs[iteration] to use for
                        // calculations later
                        diffs.push(diff);
                        curr_height += diff;

                        // Start a new piece
                        continue 'next_piece;
                    }

                    // Move the piece down
                    #[allow(clippy::cast_possible_truncation)]
                    piece
                        .iter_mut()
                        .for_each(|(_x, y)| *y = (*y as u8).wrapping_sub(1) as usize);
                }
            }
        }
    }
}

/// Print the grid of the current height
fn _print_grid(grid: &[Space], curr_height: usize, piece: &[(usize, usize)]) {
    for y in (0..20).rev() {
        let y = (curr_height - 10 + y) % GRID_HEIGHT;
        let row = &grid[y * GRID_WIDTH..(y + 1) * GRID_WIDTH];

        print!(
            "{}{:03} |",
            if curr_height % GRID_HEIGHT == y {
                "~"
            } else {
                " "
            },
            y,
        );
        for (x, space) in row.iter().enumerate() {
            match space {
                Space::Filled => print!("#"),
                Space::Empty => {
                    if piece.contains(&(x, y)) {
                        print!("@");
                    } else {
                        print!(".");
                    }
                }
            }
        }
        print!("|");
        println!();
    }
    println!();
}
