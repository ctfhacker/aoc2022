#![feature(result_option_inspect)]

use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const INPUT: &str = include_str!("../input");

/// Get the current clock cycle
fn rdtsc() -> u64 {
    unsafe { std::arch::x86_64::_rdtsc() }
}

fn run_simulation<'a>(
    core_id: usize,
    name_indexes: HashMap<&str, usize>,
    names: Vec<&str>,
    flows: Vec<usize>,
    neighbors: Vec<Vec<&str>>,
    distances: Vec<HashMap<&str, i32>>,
    best_flows: Vec<(usize, &'a str)>,
    best_score: Arc<AtomicUsize>,
    corpus: Arc<Mutex<HashSet<Vec<&'a str>>>>,
) {
    let orig_test = best_flows
        .iter()
        .filter(|(flow, name)| *name != "AA")
        .map(|(_flow, name)| *name)
        .rev()
        .collect::<Vec<_>>();

    /*
    let orig_test = vec![
        "OZ", "YI", "MZ", "UA", "AW", "YZ", "EL", "OY", "VO", "FL", "FM", "CS", "UU", "LS", "XY",
        "PE", "QX", "OQ", "OH", "QR", "YD", "OF", "CD", "DE", "ZV", "RD", "KY", "RJ", "FX", "WI",
        "GQ", "WW", "FH", "BV", "AR", "QQ", "VA", "MG", "VX", "GV", "CR", "ZE", "EG", "TU", "DY",
        "KK", "AT", "VN", "EJ", "WZ", "MC", "XL", "OR", "QD", "ZP", "XC",
    ];
    */

    let mut local_corpus = HashSet::new();
    local_corpus.insert(orig_test.clone());

    let mut local_best_score = 0;
    let mut best_steps = orig_test.clone();

    let mut start = std::time::Instant::now();
    let mut stats_timer = std::time::Instant::now();

    let mut iters = 0;

    let len = orig_test.len();
    println!("Len: {len}");

    for iters in 0..0xfffffff {
        // Stats timer every second to dump the performance of the simulations
        if stats_timer.elapsed() > std::time::Duration::from_secs(1) {
            if local_best_score > best_score.load(Ordering::SeqCst) {
                best_score.store(local_best_score, Ordering::SeqCst);
            }

            local_best_score = best_score.load(Ordering::SeqCst);

            println!(
                "{core_id} | Msims/sec: {:8.2} | Local best score: {local_best_score}",
                iters as f64 / start.elapsed().as_secs_f64() / 1024. / 1024.
            );

            // Sync local corpus with the global corpus
            if let Ok(mut corpus) = corpus.try_lock() {
                for c in &local_corpus {
                    corpus.insert(c.to_vec());
                }

                for c in corpus.iter() {
                    local_corpus.insert(c.to_vec());
                }
            }

            stats_timer = std::time::Instant::now();
        }

        // Randomly choose either to start with the best case or a corpus case
        let mut curr_test = if rdtsc() % 2 == 0 {
            local_corpus
                .iter()
                .nth(rdtsc() as usize % local_corpus.len())
                .unwrap()
                .clone()
        } else {
            best_steps.clone()
        };

        // Reset values for the simulation
        let mut minute = 1;
        let mut score = 0;
        let mut curr_increment = 0;
        let mut curr_node = "AA";
        let mut dest_node = "AA";

        // Randomly mutate the current test steps
        for _ in 0..(rdtsc() % 48 + 1) {
            let index1 = rdtsc() as usize % curr_test.len();
            let index2 = rdtsc() as usize % curr_test.len();
            if index1 == index2 {
                continue;
            }

            curr_test.swap(index1, index2);
        }

        let mut steps_left = 0;
        let mut test_index = 0;
        let mut next_increment = 0;

        // let curr_test = vec!["DD", "BB", "JJ", "HH", "EE", "CC"];

        for _minute in 0..31 {
            // println!("{minute:02} | {curr_node} -> {dest_node:?} | Dist left {steps_left:02} | Total pressure: {curr_increment}");

            score += curr_increment;

            if steps_left == 0 {
                // Destination valve is now open
                curr_increment += next_increment;
                next_increment = 0;

                if test_index < curr_test.len() {
                    // Find the next destination node
                    dest_node = curr_test[test_index];

                    // Move the index to the next
                    test_index += 1;

                    // Increment the score based the current increment
                    let curr_node_index = name_indexes[curr_node];
                    let dest_node_index = name_indexes[&*dest_node];

                    // Get the distance to the next destination node
                    let dist = distances[curr_node_index][&*dest_node];

                    //
                    steps_left = dist;

                    // Destination node is now the current node
                    curr_node = dest_node;

                    // the destination value is now open. add its pressure to the increment value per step
                    next_increment = flows[dest_node_index];
                }
            } else {
                steps_left -= 1;
            }
        }

        if score > local_best_score {
            println!("New Best Score: {score:4} || {curr_test:?}");
            local_best_score = score;
            best_steps = curr_test.clone();
            local_corpus.insert(curr_test);
        }
    }
}

fn main() {
    let mut name_indexes = HashMap::new();
    let mut names = Vec::new();
    let mut flows = Vec::new();
    let mut neighbors = Vec::new();
    let mut distances = Vec::new();
    let mut best_flows = Vec::new();

    // Valve EG has flow rate=21; tunnels lead to valves WZ, OF, ZP, QD
    for line in INPUT.lines() {
        let mut iter = line.split(' ');

        // Parse the name of the valve
        let curr_name = iter.nth(1).expect("No name?!");

        // Parse the flow rate
        let curr_flow = iter
            .nth(2)
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .replace(";", "")
            .parse::<usize>()
            .unwrap();

        // Parse each of the neighbors
        let curr_neighbors = iter
            .skip(4)
            .map(|x| x.split(',').nth(0).unwrap())
            .collect::<Vec<_>>();

        // Insert each entry into the arrays
        let curr_index = name_indexes.len();
        name_indexes.insert(curr_name, curr_index);
        names.push(curr_name);
        flows.push(curr_flow);
        best_flows.push((curr_flow, curr_name));
        neighbors.push(curr_neighbors);
    }

    let start = std::time::Instant::now();

    // Calculate the distance from each node to each other node
    for start_node in names.iter() {
        let mut seen = BTreeSet::new();
        let mut queue = vec![(*start_node, 0)];
        let mut curr_distances = HashMap::new();

        while let Some((curr_node, curr_dist)) = queue.pop() {
            curr_distances.insert(curr_node, curr_dist);

            // Mark that this node has been seen
            seen.insert(curr_node);

            let index = name_indexes.get(curr_node).unwrap();
            let curr_neighbors = &neighbors[*index];

            for neighbor in curr_neighbors {
                // If this neighbor hasn't been seen, add it to the queue
                if seen.insert(neighbor) {
                    queue.push((neighbor, curr_dist + 1));
                }
            }
        }

        distances.push(curr_distances);
    }

    println!("Calculating all distances took {:?}", start.elapsed());

    let best_score = Arc::new(AtomicUsize::new(0));
    let corpus = Arc::new(Mutex::new(HashSet::new()));
    let mut threads = Vec::new();

    for core_id in 1..8 {
        let name_indexes = name_indexes.clone();
        let names = names.clone();
        let flows = flows.clone();
        let neighbors = neighbors.clone();
        let distances = distances.clone();
        let best_flows = best_flows.clone();
        let best_score = best_score.clone();
        let corpus = corpus.clone();

        let t = std::thread::spawn(move || {
            run_simulation(
                core_id,
                name_indexes,
                names,
                flows,
                neighbors,
                distances,
                best_flows,
                best_score,
                corpus,
            )
        });

        threads.push(t);
    }

    for t in threads {
        t.join();
    }
}
