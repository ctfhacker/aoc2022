const INPUT: &[u8] = include_bytes!("../input");

/// Find the starting character of the first unique `n` bytes in `INPUT`
fn find_n_unique_bytes(n: usize, input: &[u8]) -> usize {
    'next_chunk: for curr_index in 0..input.len() {
        let mut seen = 0u64;
        for letter_index in 0..n {
            // Get the value of the
            let val = INPUT[curr_index + letter_index] - b'a';

            // Check if this letter has already been seen. If so, break out to the loop
            // to progress to the next chunk.
            if seen & (1 << val) > 0 {
                continue 'next_chunk;
            }

            // Newly seen number, mark its bit
            seen |= 1 << val;
        }

        return curr_index;
    }

    panic!("Failed to find unique {n} bytes");
}

fn main() {
    let part1 = find_n_unique_bytes(4, INPUT);

    // Found a set of 4 unique letters
    println!(
        "Part1 {}: curr_index: {:?}",
        part1 + 4,
        &INPUT[part1..part1 + 4]
    );

    let part2 = find_n_unique_bytes(14, INPUT);

    // Found a set of 14 unique letters
    println!(
        "Part2 {}: curr_index: {:?}",
        part2 + 14,
        &INPUT[part2..part2 + 14]
    );
}
