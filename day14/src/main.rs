#![feature(iter_next_chunk)]
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const INPUT: &str = include_str!("../input");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Space {
    Empty,
    Stone,
    Sand,
}

impl Space {
    fn color(&self) -> [u8; 4] {
        match self {
            Space::Empty => [0xff, 0xff, 0xff, 0xff],
            Space::Stone => [0xc0, 0xc0, 0xc0, 0xff],
            Space::Sand => [0xff, 0xe5, 0xcc, 0xff],
        }
    }
}

impl Default for Space {
    fn default() -> Self {
        Space::Empty
    }
}

fn main() {
    // Parse the coordinates as unique coordinates
    let mut coords = BTreeSet::new();

    let mut min_x = 999;
    let mut width = 0;
    let mut height = 0;

    // Parse the input into coordinates, filling in the straight lines of each endpoint
    for line in INPUT.split('\n') {
        let mut prev_coord: Option<(usize, usize)> = None;

        for curr_coords in line.split(" -> ") {
            if let Ok([x, y]) = curr_coords.split(',').next_chunk() {
                let curr_x = x.parse::<usize>().unwrap();
                let curr_y = y.parse::<usize>().unwrap();

                if let Some(prev_coord) = prev_coord {
                    let left_x = curr_x.min(prev_coord.0);
                    let right_x = curr_x.max(prev_coord.0);
                    let down_y = curr_y.min(prev_coord.1);
                    let up_y = curr_y.max(prev_coord.1);
                    for x in left_x..=right_x {
                        for y in down_y..=up_y {
                            min_x = min_x.min(x);
                            width = width.max(x);
                            height = height.max(y);
                            coords.insert((x, y));
                        }
                    }
                }

                // Reset the previous coord
                prev_coord = Some((curr_x, curr_y));
            }
        }
    }

    let coords = coords.iter().copied().collect::<Vec<_>>();

    let orig_width = width;

    for part in 1..3 {
        let sand_start_x = 500;
        let sand_start_y = 0;

        // Reset the width from the image shrinking from below
        width = orig_width;

        // Extend the width to fit part 2's challenge
        width += 300;
        height += 1;

        // Part 2 calls for a floor two lower than the lowest coord
        if part == 2 {
            height += 2;
        }

        println!("Width: {width} Height: {height}");

        // Create the grid for the given coords
        let mut grid = vec![vec![Space::Empty; width]; height];

        // Initialize the coords with Stone spaces
        for (x, y) in &coords {
            grid[*y][*x] = Space::Stone;
        }

        // For part 2, set the bottom as a line of stone
        if part == 2 {
            for x in 0..width {
                grid[height - 1][x] = Space::Stone;
            }
        }

        // Initialize the number of iterations
        let mut iters = 0;

        // Begin dropping sand
        'simulation: loop {
            let mut sand_x = sand_start_x as isize;
            let mut sand_y = sand_start_y as isize;
            if grid[sand_y as usize][sand_x as usize] == Space::Sand {
                break 'simulation;
            }

            'next_step: loop {
                for (x_mod, y_mod) in [(0, 1), (-1, 1), (1, 1)] {
                    // Check if the space under the sand is empty
                    let curr_x = sand_x + x_mod;
                    let curr_y = sand_y + y_mod;

                    if curr_y >= height as isize {
                        break 'simulation;
                    }

                    if grid[curr_y as usize][curr_x as usize] == Space::Empty {
                        sand_y += y_mod;
                        sand_x += x_mod;
                        continue 'next_step;
                    }
                }

                // Record the final resting place of the sand
                grid[sand_y as usize][sand_x as usize] = Space::Sand;

                // print_grid(&grid);

                // Sand did not progress, finished with this step
                break;
            }

            // Increase the number of iterations
            iters += 1;
        }

        println!("Width before {width}");
        // Shrink the left side
        loop {
            // If any column on the right side has sand, the image is shrunk enough
            if grid.iter().any(|line| *line.last().unwrap() == Space::Sand) {
                break;
            }

            // Remove the right column
            grid.iter_mut().for_each(|line| {
                line.pop().unwrap();
            });

            // Reduce the width by 1
            width -= 1;
        }
        println!("Width before 1 {width}");
        // Shrink the right side
        loop {
            if grid
                .iter()
                .any(|line| *line.first().unwrap() == Space::Sand)
            {
                break;
            }

            // Remove the left column
            grid.iter_mut().for_each(|line| {
                line.remove(0);
            });

            // Reduce the width by 1
            width -= 1;
        }
        println!("Width after {width}");

        println!("Part {part} Iters: {iters}");
        let filename = format!("/tmp/path{part}.png");
        let path = Path::new(&filename);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let data = grid
            .iter()
            .flatten()
            .map(|space| space.color())
            .flatten()
            .collect::<Vec<_>>();

        // println!("{data:?}");

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(data.as_slice()).unwrap();
        println!("Wrote {filename:?}");
    }
}

/// Print an ASCII representation of the simulation grid
fn _print_grid(grid: &[Vec<Space>]) {
    for (index, line) in grid.iter().enumerate() {
        print!("{index:2} ");
        for space in line {
            match space {
                Space::Empty => print!(" "),
                Space::Stone => print!("#"),
                Space::Sand => print!("o"),
            }
        }
        println!();
    }
}
