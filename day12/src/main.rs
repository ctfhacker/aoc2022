use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("../input");

fn main() {
    for start_with_a in [false, true] {
        let mut grid = INPUT
            .split(|x| *x == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect::<Vec<Vec<_>>>();

        let width = grid.first().unwrap().len() as isize;
        let height = grid.len() as isize;

        // Find the ending space to work backwards
        let mut queue = Vec::new();
        let mut end = None;

        for (y, line) in grid.iter_mut().enumerate() {
            for (x, ch) in line.iter_mut().enumerate() {
                if *ch == b'S' {
                    queue.push((b'a' - 1, 0, (x as isize, y as isize)));

                    // Reset the starting position to the byte before b'a' to let the first path
                    // movement be checked in the same manner as all other letters
                    *ch = b'a' - 1;
                }

                if start_with_a && *ch == b'a' {
                    queue.push((b'a', 0, (x as isize, y as isize)));
                }

                if *ch == b'E' {
                    *ch = b'z' + 1;
                    end = Some((x as isize, y as isize));
                }
            }
        }

        let mut distances = HashMap::new();

        // Initialize the distances lookup
        for y in 0..grid.len() {
            for x in 0..grid.first().unwrap().len() {
                distances.insert((x as isize, y as isize), 999);
            }
        }

        // let mut seen = HashSet::new();

        while let Some((curr_byte, curr_dist, (curr_x, curr_y))) = queue.pop() {
            for (x_mod, y_mod) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = curr_x + x_mod;
                let new_y = curr_y + y_mod;

                // Ensure the lookup coordinate is in bounds
                if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                    continue;
                }

                // Ensure going to this new space will decrease the current path length
                if *distances.get(&(new_x, new_y)).unwrap() <= curr_dist + 1 {
                    continue;
                }

                // Ensure the next character is, at most, one step away from the current byte
                let check_byte = grid[new_y as usize][new_x as usize];
                if check_byte > curr_byte + 1 {
                    continue;
                }

                // Write the new best distance to this coord
                distances.insert((new_x, new_y), curr_dist + 1);

                // Add the new best coord to the queue
                queue.push((check_byte, curr_dist + 1, (new_x, new_y)));
            }
        }

        println!(
            "Starting with A {start_with_a:5} path len -- {:?}",
            *distances.get(&end.unwrap()).unwrap() - 2
        );
    }
}
