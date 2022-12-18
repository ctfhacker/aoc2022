use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input");

enum Direction {
    /// Y + 1
    Up,

    /// Y - 1
    Down,

    /// X - 1
    Left,

    /// X + 1
    Right,

    /// Z + 1
    Front,

    /// Z - 1
    Back,
}

fn main() {
    // Part the input coordinates
    let boxes = INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Calculate the minumum and maximum coords for bounding the search space
    let min = boxes.iter().flatten().min().unwrap();
    let max = boxes.iter().flatten().max().unwrap();

    println!("Min: {min:?} Max: {max:?}");

    let mut exposed_sides = 0;

    for coord in &boxes {
        let [x, y, z] = coord.as_slice()[..3] else { panic!("{coord:?}") };

        let mut curr_exposed_sides = 0;

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Front,
            Direction::Back,
        ] {
            let check_coord = match dir {
                Direction::Up => [x, y + 1, z],
                Direction::Down => [x, y - 1, z],
                Direction::Left => [x - 1, y, z],
                Direction::Right => [x + 1, y, z],
                Direction::Front => [x, y, z + 1],
                Direction::Back => [x, y, z - 1],
            }
            .to_vec();

            // Check for neighboring box
            if boxes.contains(&check_coord) {
                continue;
            } else {
                curr_exposed_sides += 1;
                continue;
            }
        }

        exposed_sides += curr_exposed_sides;
    }

    println!("Part 1 sides: {exposed_sides}");

    // Initialize the 3d search queue
    let mut queue: VecDeque<Vec<i32>> = VecDeque::new();
    let mut seen = HashSet::new();
    let mut starting_point = boxes.first().unwrap().clone();

    // Find the first box that is adjacent to the water droplet but not a part of it
    for nth in 0..boxes.len() {
        let this_nth = boxes.get(nth).unwrap().clone();

        // Attempt to start the search just outside of the first box
        for offset in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let mut check = this_nth.clone();
            check[0] += offset.0;
            check[1] += offset.1;
            check[2] += offset.2;

            if !boxes.contains(&check) {
                starting_point = check;
                break;
            }
        }
    }

    queue.push_back(starting_point);

    let mut exposed_sides = 0;

    // Walk aroudn the external of the water droplet counting the exposed sides
    while let Some(coord) = queue.pop_front() {
        assert!(
            !boxes.contains(&coord),
            "Search box is in the input boxes?!"
        );

        // Don't walk over the same coord twice
        if !seen.insert(coord.clone()) {
            continue;
        }

        // Bound the search square to just next to the main input
        if *coord.iter().min().unwrap() < min - 1 {
            continue;
        }

        // Bound the search square to just next to the main input
        if *coord.iter().max().unwrap() > max + 1 {
            continue;
        }

        // Parse the coords
        let [x, y, z] = coord.as_slice()[..3] else { panic!("{coord:?}") };

        // Initialize the total exposed sides for this search box
        let mut curr_exposed_sides = 0;

        // Check each direction from the current search square. If there is an input box there,
        // then that side is exposed externally. If not, add the empty square to search next
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Front,
            Direction::Back,
        ] {
            let check_coord = match dir {
                Direction::Up => [x, y + 1, z],
                Direction::Down => [x, y - 1, z],
                Direction::Left => [x - 1, y, z],
                Direction::Right => [x + 1, y, z],
                Direction::Front => [x, y, z + 1],
                Direction::Back => [x, y, z - 1],
            }
            .to_vec();

            // Check for neighboring box. If the neighbor side is an input box, it is an exposed side
            if boxes.contains(&check_coord) {
                curr_exposed_sides += 1;
            } else {
                // This side is not a part of the water droplet. Add it as part of the search path
                queue.push_back(check_coord);
            }
        }

        // This search point is done, add it's found sides to the total number of external sides
        exposed_sides += curr_exposed_sides;
    }

    println!("Part 2 sides: {exposed_sides}");
}
