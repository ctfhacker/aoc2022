const INPUT: &[u8] = include_bytes!("../input");

fn part1() {
    let lines: Vec<_> = INPUT
        .split(|x| *x == b'\n')
        .filter(|line| !line.is_empty())
        .collect();

    let mut visible = Vec::new();
    let height = lines.len();

    'next_row: for (row, line) in lines.iter().enumerate() {
        let width = line.len();

        if (row == 0) || (row == lines.len() - 1) {
            // Every tree on the top and bottom rows are visible. Fast path here to
            // allocate all trees being visible and continue to the next line
            visible.push(vec![true; width]);
            continue 'next_row;
        } else {
            // Allocate a visible state for each tree in this line
            visible.push(vec![false; width]);
        }

        'next_col: for (column, curr_tree) in line.iter().enumerate() {
            if column == 0 || column == width - 1 {
                // Trees on the first and last column are always visible. Mark them as
                // such
                visible[row][column] = true;
                continue 'next_col;
            }

            // Naively search the four directions to see if this tree can be seen

            // Search down
            if (row + 1..height).all(|curr_row| lines[curr_row][column] < *curr_tree) {
                // Found a visible path down, mark as visible and move into the next tree
                // since we only need one visible path
                visible[row][column] = true;
                continue 'next_col;
            }

            // Search left
            if (0..column)
                .rev()
                .all(|curr_col| lines[row][curr_col] < *curr_tree)
            {
                // Found a visible path left, mark as visible and move into the next tree
                // since we only need one visible path
                visible[row][column] = true;
                continue 'next_col;
            }

            // Search right
            if (column + 1..width).all(|curr_col| lines[row][curr_col] < *curr_tree) {
                // Found a visible path right, mark as visible and move into the next tree
                // since we only need one visible path
                visible[row][column] = true;
                continue 'next_col;
            }

            // Search up
            if (0..row)
                .rev()
                .all(|curr_row| lines[curr_row][column] < *curr_tree)
            {
                // Found a visible path up, mark as visible and move into the next tree
                // since we only need one visible path
                visible[row][column] = true;
                continue 'next_col;
            }
        }
    }

    println!(
        "Visible: {}",
        visible.iter().flatten().filter(|x| **x).count()
    );
}

fn part2() {
    let lines: Vec<_> = INPUT
        .split(|x| *x == b'\n')
        .filter(|line| !line.is_empty())
        .collect();

    let mut scenic_scores = Vec::new();
    let height = lines.len();

    for (row, line) in lines.iter().enumerate() {
        let width = line.len();

        // Initialize the scenic scores
        scenic_scores.push(vec![(0, 0, 0, 0); width]);

        for (column, curr_tree) in line.iter().enumerate() {
            // Search left
            let mut score_left = 0;
            for curr_col in (0..column).rev() {
                // Every tree seen in the right direction counts
                score_left += 1;

                // If the current tree is taller or equal to the current tree, this is
                // the end of the eave
                if lines[row][curr_col] >= *curr_tree {
                    break;
                }
            }

            // Search right
            let mut score_right = 0;
            for curr_col in column + 1..width {
                // Every tree seen in the right direction counts
                score_right += 1;

                // If the current tree is taller or equal to the current tree, this is
                // the end of the eave
                if lines[row][curr_col] >= *curr_tree {
                    break;
                }
            }

            // Search up
            let mut score_up = 0;
            for curr_row in (0..row).rev() {
                // Every tree seen in the up direction counts
                score_up += 1;

                // If the current tree is taller or equal to the current tree, this is
                // the end of the eave
                if lines[curr_row][column] >= *curr_tree {
                    break;
                }
            }

            // Search down
            let mut score_down = 0;

            // Keep the same index form for all directions even if this loop could be
            // written slightly differently according to clippy
            #[allow(clippy::needless_range_loop)]
            for curr_row in row + 1..height {
                // Every tree seen in the down direction counts
                score_down += 1;

                // If the current tree is taller or equal to the current tree, this is
                // the end of the eave
                if lines[curr_row][column] >= *curr_tree {
                    break;
                }
            }

            // Save the individual scores of each direction for debugging
            scenic_scores[row][column] = (score_left, score_right, score_up, score_down);
        }
    }

    // Calculate each of the scenic scores for all trees and get the best one
    println!(
        "Best scenic score: {:?}",
        scenic_scores
            .iter()
            .flatten()
            .map(|(a, b, c, d)| a * b * c * d)
            .max()
    );
}

fn main() {
    part1();
    part2();
}
