use std::collections::HashMap;
use std::path::PathBuf;

const INPUT: &str = include_str!("../input");

fn main() {
    let mut curr_dir = PathBuf::from("/");
    let mut curr_sizes = HashMap::new();

    // Parse the input lines
    for line in INPUT.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            // Ignore the `ls` command and `dir` commands
            continue;
        } else if line.starts_with("$ cd") {
            // Expected line format: $ cd bsnqsfm

            // Get the directory from the line
            let directory = line.split(' ').nth(2).unwrap();
            match &directory {
                // Reset the path to root
                &"/" => {
                    curr_dir = PathBuf::from("/");
                }
                // Go up a directory to the parent
                &".." => {
                    curr_dir.pop();
                }
                // Add the given directory to the current path
                dir => {
                    curr_dir = curr_dir.join(dir);
                }
            }
        } else {
            // Expected line format: 221336 gdjfp.mfp
            let size = line.split(' ').next().unwrap().parse::<u32>().unwrap();

            // Clone the current working directory in preparation of adding the current
            // file size to all parent directories
            let mut tmp_dir = curr_dir.clone();
            *curr_sizes.entry(tmp_dir.clone()).or_insert(0) += size;

            // Add the current file size to each parent directory
            while tmp_dir.pop() {
                *curr_sizes.entry(tmp_dir.clone()).or_insert(0) += size;
            }
        }
    }

    // Calculate the size of all directories under 100_000 bytes
    let sum: u32 = curr_sizes
        .iter()
        .filter_map(|(_, size)| if *size <= 100000 { Some(size) } else { None })
        .sum();

    println!("Part 1: {sum}");

    // Get the total size of the entire filesystem from /
    let total_size = curr_sizes
        .get(&PathBuf::from("/"))
        .expect("Root directory ('/') not found");

    // Calculate the space needed at minimum to reach the 30_000_000 bytes needed
    let size_needed = 30_000_000 - (70_000_000 - total_size);
    let mut curr_best = u32::MAX;

    // Find the smallest directory that would free up enough space
    for size in curr_sizes.values() {
        if *size >= size_needed && *size < curr_best {
            curr_best = *size;
        }
    }

    println!("Part 2: total size: {total_size} Best: {curr_best}");
}
