const INPUT: &str = include_str!("../input");

// -357
// 7174

#[derive(Debug)]
struct Number {
    initial_index: usize,
    value: isize,
}

fn main() {
    let mut nums = INPUT
        .split('\n')
        .filter(|x| !x.is_empty())
        .enumerate()
        .map(|(index, x)| Number {
            initial_index: index,
            value: x.parse::<isize>().unwrap() * 811589153,
        })
        .collect::<Vec<_>>();

    // println!("{:?}", nums.iter().map(|x| x.value).collect::<Vec<_>>());

    // Reset the numbers with the initial indexes
    nums.iter_mut()
        .enumerate()
        .for_each(|(index, value)| value.initial_index = index);

    for iter in 0..10 {
        // Move each value once in order of the original sequence
        for curr_index in 0..nums.len() {
            // Find the value of the next item in the list
            let mut list_index = None;
            for i in 0..nums.len() {
                if nums[i].initial_index == curr_index {
                    list_index = Some(i);
                    break;
                }
            }

            // Get the current index of the next value to move
            let Some(remove_index) = list_index else { panic!("Did not find index needed") };

            // Remove the value from the list
            let value = nums.remove(remove_index);

            let mut new_index = remove_index as isize;
            let old_index = new_index;

            /*
            // Manually implement the movement to use as a check for the single
            if value.value > 0 {
                for _ in 0..value.value.abs() {
                    if new_index == nums.len() as isize {
                        new_index = 1;
                    } else {
                        new_index += 1;
                    }
                }
            } else if value.value < 0 {
                for _ in 0..value.value.abs() {
                    if new_index == 0 {
                        new_index = nums.len() as isize - 1;
                    } else {
                        new_index -= 1;
                    }
                }
            }

            println!(
                "Remove index: {old_index} Value: {:?} New index: {new_index} Nums len: {}",
                value.value,
                nums.len()
            );

            // Sanity check the `test_val` index matches the manual single stepping to understand
            // how wrapping should work
            assert!(
                test_val == new_index as usize,
                "test_val: {test_val} new_index: {new_index}"
            );
            */

            let mut test_val =
                (old_index + nums.len() as isize + (value.value % nums.len() as isize)) as usize
                    % nums.len() as usize;

            // When moving with a positive value that ends with a wrap to `0`, the value should
            // be the end of the list instead
            if test_val == 0 && value.value > 0 {
                test_val = nums.len();
            }

            // Set the new index to the calculated value
            new_index = test_val as isize;

            // println!("Inserting at {new_index}");
            nums.insert(new_index as usize, value);
        }
    }

    // Find where the zeroth value currently is in the sequence
    let mut zero_index = None;
    for index in 0..nums.len() {
        if nums[index].value != 0 {
            continue;
        }

        zero_index = Some(index);
        break;
    }

    let mut sum = 0;
    let index = zero_index.unwrap();
    for offset in [1000, 2000, 3000] {
        sum += nums[(index + offset) % nums.len()].value;
    }

    println!("Part 2: {sum}");
}
