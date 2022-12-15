use rand::prelude::*;

const INPUT: &str = include_str!("/home/user/workspace/chris-aoc2022/2022/15/input.txt");
// const INPUT: &str = include_str!("../input");

/// Calculate the Manhattan distance
fn manhattan_distance(first: (isize, isize), second: (isize, isize)) -> isize {
    (first.0.max(second.0) - first.0.min(second.0)).abs()
        + (first.1.max(second.1) - first.1.min(second.1)).abs()
}

fn rdtsc() -> u64 {
    unsafe { std::arch::x86_64::_rdtsc() }
}

fn main() {
    let wanted_row = 2000000;

    let mut min_x = 0;
    let mut max_x = 0;

    let mut pairs = Vec::new();

    let start = std::time::Instant::now();

    // Parse the sensor and beacon coordinates
    for line in INPUT.lines().filter(|line| !line.is_empty()) {
        // Sensor at x=1326566, y=3575946: closest beacon is at x=1374835, y=2000000
        let mut iter = line.split(' ').skip(2);
        let sensor_x = iter.next().unwrap()[2..]
            .replace(',', "")
            .parse::<isize>()
            .unwrap();

        let sensor_y = iter.next().unwrap()[2..]
            .replace(':', "")
            .parse::<isize>()
            .unwrap();

        // Skip over `closest beacon is at`
        let mut iter = iter.skip(4);

        let beacon_x = iter.next().unwrap()[2..]
            .replace(',', "")
            .parse::<isize>()
            .unwrap();

        let beacon_y = iter.next().unwrap()[2..].parse::<isize>().unwrap();

        pairs.push(((sensor_x, sensor_y), (beacon_x, beacon_y)));
    }

    let mut missing_beacon_x = 2000000;
    let mut missing_beacon_y = 2000000;

    let mut rng = rand::thread_rng();

    // Start the missing beacon of the grid at (2000000, 2000000).
    // Look to see if the current missing beacon is in range of the current sensor.
    // If it is, move the missing beacon to just out of range of the current sensor.
    // Repeat this process until the beacon is outside the range of all sensors
    loop {
        let mut beacon_moved = false;

        // Shuffle the ordering of the signal/beacon pairs to ensure we aren't caught in a loop
        pairs.shuffle(&mut rng);

        for ((sensor_x, sensor_y), (beacon_x, beacon_y)) in &pairs {
            // Calculate the manhattan distance to know the maximum distance from this sensor
            let distance = manhattan_distance((*sensor_x, *sensor_y), (*beacon_x, *beacon_y));

            // println!("{sensor_x},{sensor_y} | {beacon_x},{beacon_y} || {distance}");

            // Check if this sensor can reach the requested row
            let distance_to_wanted_row = (wanted_row - sensor_y).abs();

            // Solve for part 1
            if distance_to_wanted_row <= distance {
                let curr_min_x = sensor_x - (distance - distance_to_wanted_row);
                let curr_max_x = sensor_x + (distance - distance_to_wanted_row);

                min_x = min_x.min(curr_min_x);
                max_x = max_x.max(curr_max_x);
            }

            if (*sensor_x, *sensor_y) == (missing_beacon_x, missing_beacon_y) {
                missing_beacon_x += 1;
                // missing_beacon_x += 2;
            }

            // Check if the missing beacon is in range of this sensor. If so, move it out.
            let missing_beacon_distance =
                manhattan_distance((*sensor_x, *sensor_y), (missing_beacon_x, missing_beacon_y));

            // If the missing beacon is in range of this sensor, move it to just outside the
            // beacon's range
            if missing_beacon_distance <= distance {
                let vert_dist = (missing_beacon_y - sensor_y).abs();
                let horiz_dist = (missing_beacon_x - sensor_x).abs();
                let movement = (distance - vert_dist - horiz_dist).max(1);

                if (movement == 1 && rdtsc() % 4 == 0) || movement > 1 {
                    if *sensor_x > missing_beacon_x && missing_beacon_x > movement {
                        // Move the test point left
                        missing_beacon_x -= movement;
                    } else if *sensor_x < missing_beacon_x
                        && (missing_beacon_x + movement) <= 4000000
                    {
                        // Move the test point right
                        missing_beacon_x += movement;
                    }
                }

                if (movement == 1 && rdtsc() % 4 == 0) || movement > 1 {
                    if *sensor_y > missing_beacon_y && missing_beacon_y > movement {
                        // Move the test point up
                        missing_beacon_y -= movement;
                    } else if *sensor_y < missing_beacon_y
                        && (missing_beacon_y + movement) <= 4000000
                    {
                        // Move the test point down
                        missing_beacon_y += movement;
                    }
                }

                missing_beacon_x = missing_beacon_x.clamp(0, 4000000);
                missing_beacon_y = missing_beacon_y.clamp(0, 4000000);

                beacon_moved = true;
            }
        }

        // Found a spot where the beacon didn't move
        if !beacon_moved {
            break;
        }
    }

    println!("Part 1 Row: {}", max_x - min_x);

    println!(
        "Part 2 Missing beacon: {}",
        missing_beacon_x * 4000000 + missing_beacon_y
    );

    println!("Elapsed: {:?}", start.elapsed());
}
