#![feature(iter_next_chunk)]
#![feature(const_mut_refs)]
#![feature(const_option)]

/// The puzzle input
const INPUT: &[u8] = include_bytes!("../input");

/// The number of pre-allocated columns
const NUMBER_OF_COLUMNS: usize = 9;

/// The max height of all columns
const COLUMN_HEIGHT: usize = 42;

const fn parse_input_setup() -> (
    [[Option<u8>; COLUMN_HEIGHT]; NUMBER_OF_COLUMNS],
    [usize; NUMBER_OF_COLUMNS],
    usize,
) {
    let mut input_index = 0;
    let mut col_index = 0;

    // Allocate 10 columns that can contain COLUMN_HEIGHT elements each
    let mut cols = [[None; COLUMN_HEIGHT]; NUMBER_OF_COLUMNS];

    // The current next index to insert an element for each column
    let mut col_next_index = [COLUMN_HEIGHT - 1; NUMBER_OF_COLUMNS];

    // Parse the initial input column state
    loop {
        // Found the double newline between the setup and the instructions
        if INPUT[input_index] == b'\n' && INPUT[input_index + 1] == b'\n' {
            input_index += 2;
            break;
        }

        // Get the current byte of the index
        let curr_byte = INPUT[input_index];

        if input_index % 4 == 1 {
            if curr_byte.is_ascii_uppercase() {
                let curr_col_next_index = &mut col_next_index[col_index];
                cols[col_index][*curr_col_next_index] = Some(curr_byte);

                // Decrement the next index of the current column since we are reading
                // the columns from top to bottom
                *curr_col_next_index -= 1;
            }

            // Increment the column index
            col_index += 1;

            if col_index > NUMBER_OF_COLUMNS {
                panic!("Too many columns. Increment the NUMBER_OF_COLUMNS const variable");
            }
        }

        // If we hit a newline, reset the column index
        if curr_byte == b'\n' {
            col_index = 0;
        }

        // Increment the input index
        input_index += 1;
    }

    // Currently, all of the values are at the end of each array. We want to move all
    // of the values to the beginning of each array so that the bottom of each column is
    // at index 0
    let mut column = 0;
    loop {
        if column >= NUMBER_OF_COLUMNS {
            break;
        }

        // +1 here since the col_next_index always points to the next index to write to.
        let curr_col_index = col_next_index[column] + 1;

        // Reset the column indexes to point to the index just past the top of each
        // column after inversion
        let new_col_index = COLUMN_HEIGHT - curr_col_index;
        col_next_index[column] = new_col_index;

        let mut index = curr_col_index;
        loop {
            // If the index is beyond the maximum height, we are finished with this
            // column
            if index >= COLUMN_HEIGHT {
                break;
            }

            // Take the value from the end of the array (leaving the None) and move it to
            // the beginning of the array
            let curr_value = cols[column][index].take();
            cols[column][index - curr_col_index] = curr_value;

            // Increment to the next index
            index += 1;
        }

        // Go to the next column
        column += 1;
    }

    (cols, col_next_index, input_index)
}

const fn part1() -> [char; NUMBER_OF_COLUMNS] {
    let (mut cols, mut col_next_index, mut input_index) = parse_input_setup();

    // Now that the columns are parsed and at the beginning of each array, we can begin
    // executing the `move` instructions
    loop {
        if input_index >= INPUT.len() {
            break;
        }

        // Sanity check we are at the beginning of each parsed line
        assert!(
            INPUT[input_index] == b'm'
                && INPUT[input_index + 1] == b'o'
                && INPUT[input_index + 2] == b'v'
                && INPUT[input_index + 3] == b'e'
                && INPUT[input_index + 4] == b' ',
            "`move ` not found at the beginning of the instruction"
        );

        // Increment past `move`
        input_index += 5;

        let val1 = INPUT[input_index];

        //
        input_index += 1;

        let val2 = INPUT[input_index];

        // Parse the one or two digit count value
        let mut count = match val2 {
            b' ' => {
                // Found a single digit count. Use val1
                val1 - b'0'
            }
            b'0'..=b'9' => {
                // Found a two digit count. Use val2 val1
                input_index += 1;

                let val2 = val2 - b'0';
                let val1 = val1 - b'0';

                val1 * 10 + val2
            }
            _ => {
                panic!("Invalid `move` instruction line");
            }
        };

        assert!(
            INPUT[input_index] == b' '
                && INPUT[input_index + 1] == b'f'
                && INPUT[input_index + 2] == b'r'
                && INPUT[input_index + 3] == b'o'
                && INPUT[input_index + 4] == b'm'
                && INPUT[input_index + 5] == b' ',
            "` from ` not found in instruction line"
        );

        // Increment past the ` from `
        input_index += 6;

        // -1 here since all columns are "1-indexed in the puzzle"
        let src_col = (INPUT[input_index] - b'0') as usize - 1;
        input_index += 1 + " to ".len();
        let dst_col = (INPUT[input_index] - b'0') as usize - 1;

        loop {
            if count == 0 {
                break;
            }

            // Take the value from the source column and put it into the dest column
            col_next_index[src_col] -= 1;
            let src_col_height = col_next_index[src_col];
            let value = cols[src_col][src_col_height].take();
            assert!(value.is_some(), "Attempted to move a None value");

            let dst_col_height = col_next_index[dst_col];
            col_next_index[dst_col] += 1;
            cols[dst_col][dst_col_height] = value;

            count -= 1;
        }

        // Increment past the dst_col
        input_index += 1;

        assert!(
            INPUT[input_index] == b'\n',
            "Newline not found at the end of the line"
        );

        // Increment past the newline
        input_index += 1;
    }

    let mut col_index = 0;
    let mut solution = ['?'; NUMBER_OF_COLUMNS];
    loop {
        if col_index >= NUMBER_OF_COLUMNS {
            break;
        }

        let curr_height = col_next_index[col_index] - 1;
        solution[col_index] = cols[col_index][curr_height].unwrap() as char;

        col_index += 1;
    }

    solution
}

const fn part2() -> [char; NUMBER_OF_COLUMNS] {
    let (mut cols, mut col_next_index, mut input_index) = parse_input_setup();

    // Now that the columns are parsed and at the beginning of each array, we can begin
    // executing the `move` instructions
    loop {
        if input_index >= INPUT.len() {
            break;
        }

        // Sanity check we are at the beginning of each parsed line
        assert!(
            INPUT[input_index] == b'm'
                && INPUT[input_index + 1] == b'o'
                && INPUT[input_index + 2] == b'v'
                && INPUT[input_index + 3] == b'e'
                && INPUT[input_index + 4] == b' ',
            "`move ` not found at the beginning of the instruction"
        );

        // Increment past `move`
        input_index += 5;

        let val1 = INPUT[input_index];

        //
        input_index += 1;

        let val2 = INPUT[input_index];

        // Parse the one or two digit count value
        let mut count = match val2 {
            b' ' => {
                // Found a single digit count. Use val1
                val1 - b'0'
            }
            b'0'..=b'9' => {
                // Found a two digit count. Use val2 val1
                input_index += 1;

                let val2 = val2 - b'0';
                let val1 = val1 - b'0';

                val1 * 10 + val2
            }
            _ => {
                panic!("Invalid `move` instruction line");
            }
        };

        assert!(
            INPUT[input_index] == b' '
                && INPUT[input_index + 1] == b'f'
                && INPUT[input_index + 2] == b'r'
                && INPUT[input_index + 3] == b'o'
                && INPUT[input_index + 4] == b'm'
                && INPUT[input_index + 5] == b' ',
            "` from ` not found in instruction line"
        );

        // Increment past the ` from `
        input_index += 6;

        // -1 here since all columns are "1-indexed in the puzzle"
        let src_col = (INPUT[input_index] - b'0') as usize - 1;
        input_index += 1 + " to ".len();
        let dst_col = (INPUT[input_index] - b'0') as usize - 1;

        let init_count = count;
        loop {
            if count == 0 {
                break;
            }

            // Take the value from the source column and put it into the dest column
            let src_col_height = col_next_index[src_col] - count as usize;

            let dst_col_height = col_next_index[dst_col];
            col_next_index[dst_col] += 1;

            let value = cols[src_col][src_col_height].take();
            cols[dst_col][dst_col_height] = value;

            count -= 1;
        }

        // We can now adjust the source index now that the copy has finished
        col_next_index[src_col] -= init_count as usize;

        // Increment past the dst_col
        input_index += 1;

        // Sanity check we made it to the end of the line
        assert!(
            INPUT[input_index] == b'\n',
            "Newline not found at the end of the line"
        );

        // Increment past the newline
        input_index += 1;
    }

    let mut col_index = 0;
    let mut solution = ['?'; NUMBER_OF_COLUMNS];
    loop {
        if col_index >= NUMBER_OF_COLUMNS {
            break;
        }

        let curr_height = col_next_index[col_index] - 1;
        solution[col_index] = cols[col_index][curr_height].unwrap() as char;

        col_index += 1;
    }

    solution
}

fn main() {
    const PART1_SOLUTION: [char; 9] = part1();
    println!("{PART1_SOLUTION:?}");

    const PART2_SOLUTION: [char; 9] = part2();
    println!("{PART2_SOLUTION:?}");
}
