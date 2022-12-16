#![feature(result_option_inspect)]

use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const INPUT: &str = include_str!("../input");

/// Get the current clock cycle
fn rdtsc() -> u64 {
    unsafe { std::arch::x86_64::_rdtsc() }
}

fn run_simulation<'a>(
    core_id: usize,
    name_indexes: Arc<HashMap<&str, usize>>,
    names: Arc<Vec<&str>>,
    flows: Arc<Vec<usize>>,
    neighbors: Arc<Vec<Vec<&str>>>,
    distances: Arc<Vec<HashMap<usize, usize>>>,
    best_score: Arc<AtomicUsize>,
    corpus: Arc<Mutex<HashSet<Vec<usize>>>>,
    max_time: isize,
    valuables: BTreeSet<usize>,
) {
    let orig_test = valuables.iter().copied().collect::<Vec<_>>();

    let mut local_corpus: HashSet<Vec<usize>> = HashSet::new();
    local_corpus.insert(orig_test.clone());

    let mut local_best_score = 0;
    let mut best_steps = orig_test.clone();

    let mut start = std::time::Instant::now();
    let mut stats_timer = std::time::Instant::now();

    let mut iters = 0;

    for iters in 0..0x2ffff {
        // Stats timer every second to dump the performance of the simulations
        if stats_timer.elapsed() > std::time::Duration::from_secs(5) {
            if local_best_score > best_score.load(Ordering::SeqCst) {
                best_score.store(local_best_score, Ordering::SeqCst);
            }

            local_best_score = best_score.load(Ordering::SeqCst);

            /*
            println!(
                "{core_id} | Msims/sec: {:8.2} | Local best score: {local_best_score}",
                iters as f64 / start.elapsed().as_secs_f64() / 1024. / 1024.
            );
            */

            // Sync local corpus with the global corpus
            if let Ok(mut corpus) = corpus.try_lock() {
                for c in &local_corpus {
                    corpus.insert(c.iter().map(|x| *x).collect());
                }

                for c in corpus.iter() {
                    local_corpus.insert(c.clone());
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

        // Randomly mutate the current test steps
        for _ in 0..(rdtsc() % 64 + 1) {
            let index1 = rdtsc() as usize % curr_test.len();
            let index2 = rdtsc() as usize % curr_test.len();
            if index1 == index2 {
                continue;
            }

            curr_test.swap(index1, index2);
        }

        // Reset values for the simulation
        let mut score = 0;

        let mut time_left = max_time;
        let mut curr_node = name_indexes["AA"];

        // Get the valuable destination node's not currently in the path
        for dest_index in &curr_test {
            let dist = distances[curr_node][&dest_index];

            // Each distance takes one time and turning on the valve takes one time
            time_left -= dist as isize + 1;

            // Cannot reach this destination in enough time to turn on the valve
            if time_left <= 0 {
                continue;
            }

            // This valve will score its flow each remaining time step
            let flow = flows[*dest_index];
            score = score + time_left as usize * flow;

            // Update the current node
            curr_node = *dest_index;
        }

        if score > local_best_score {
            // println!("New Best Score: {score:4} || {curr_test:?}");
            local_best_score = score;
            best_steps = curr_test.clone();
            local_corpus.insert(curr_test);
        }
    }

    if local_best_score > best_score.load(Ordering::SeqCst) {
        best_score.store(local_best_score, Ordering::SeqCst);
    }
}

fn naive(
    name_indexes: HashMap<&str, usize>,
    names: Vec<&str>,
    flows: Vec<usize>,
    distances: Vec<HashMap<usize, usize>>,
    valuables: BTreeSet<usize>,
    max_time: isize,
) -> usize {
    let starting_node = "AA";
    let starting_index = name_indexes[starting_node];
    let mut queue = VecDeque::new();
    let curr_path: Vec<usize> = vec![starting_index];
    queue.push_back((starting_index, curr_path, max_time, 0));

    let mut best_score = 0;

    while let Some((curr_node, curr_path, time_left, score)) = queue.pop_front() {
        if score > best_score {
            best_score = score;
        }

        // Get the valuable destination node's not currently in the path
        for dest_index in &valuables {
            if curr_path.contains(dest_index) {
                continue;
            }

            let dist = distances[curr_node][dest_index];

            // Each distance takes one time and turning on the valve takes one time
            let curr_time_left = time_left - dist as isize - 1;

            // Cannot reach this destination in enough time to turn on the valve
            if curr_time_left <= 0 {
                continue;
            }

            // This valve will score its flow each remaining time step
            let flow = flows[*dest_index];
            let curr_score = score + (curr_time_left as usize * flow);

            // Create the new path containing this destination
            let mut new_path = curr_path.clone();
            new_path.push(*dest_index);
            queue.push_back((*dest_index, new_path, curr_time_left, curr_score));
        }
    }

    println!("Best score: {best_score} | Valuables: {}", valuables.len());

    best_score
}

fn main() {
    let mut name_indexes = HashMap::new();
    let mut names: Vec<&str> = Vec::new();
    let mut flows: Vec<usize> = Vec::new();
    let mut neighbors: Vec<Vec<&str>> = Vec::new();
    // let mut best_flows = Vec::new();

    // Valve EG has flow rate=21; tunnels lead to valves WZ, OF, ZP, QD
    for line in INPUT.lines() {
        let mut iter = line.split(' ');

        // Parse the name of the valve
        let curr_name = iter.nth(1).expect("No name?!");

        // Parse the flow rate
        let curr_flow = iter
            .nth(2)
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .replace(';', "")
            .parse::<usize>()
            .unwrap();

        // Parse each of the neighbors
        let curr_neighbors = iter
            .skip(4)
            .map(|x| x.split(',').next().unwrap())
            .collect::<Vec<_>>();

        // println!("{curr_name}: Neighbors: {curr_neighbors:?} Flow: {curr_flow}");

        // Insert each entry into the arrays
        let curr_index = name_indexes.len();
        name_indexes.insert(curr_name, curr_index);
        names.push(curr_name);
        flows.push(curr_flow);
        neighbors.push(curr_neighbors);
    }

    let start = std::time::Instant::now();

    let mut distances = Vec::new();

    // Calculate the distance from each node to each other node
    for start_node_index in 0..names.len() {
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_node_index, 0_usize));
        let mut curr_distances = HashMap::new();

        while let Some((curr_node, curr_dist)) = queue.pop_front() {
            if curr_distances.insert(curr_node, curr_dist).is_some() {
                panic!();
            }

            seen.insert(curr_node);

            let curr_neighbors = &neighbors[curr_node];

            for neighbor in curr_neighbors {
                // If this neighbor hasn't been seen, add it to the queue
                let neighbor_index = name_indexes[neighbor];

                if seen.insert(neighbor_index) {
                    queue.push_back((neighbor_index, curr_dist + 1));
                }
            }
        }

        distances.push(curr_distances);
    }

    println!("Calculating all distances took {:?}", start.elapsed());

    // Get the valves that have a flow rate as the target destinations
    let valuables = flows
        .iter()
        .enumerate()
        .filter_map(|(index, flow)| (names[index] != "AA" && *flow > 0).then_some(index))
        .collect::<BTreeSet<_>>();

    /*
    for (i, k) in distances.iter().enumerate() {
        let name = names[i];
        if name == "AA" {
            print!("{name} | ");
            for (k, d) in k {
                print!("{}={d}, ", names[*k]);
            }
            println!();
        }
    }
    */

    /*
    naive(
        name_indexes.clone(),
        names.clone(),
        flows.clone(),
        distances.clone(),
        valuables,
        30,
    );
    */

    let best_score = Arc::new(AtomicUsize::new(0));
    let corpus = Arc::new(Mutex::new(HashSet::new()));
    let mut threads = Vec::new();
    let orig_test = flows
        .iter()
        .enumerate()
        .filter_map(|(index, flow)| (names[index] != "AA" && *flow > 0).then_some(index))
        .collect::<BTreeSet<_>>();

    // Get the read-only variables ready for threads
    let name_indexes = Arc::new(name_indexes);
    let names = Arc::new(names);
    let flows = Arc::new(flows);
    let neighbors = Arc::new(neighbors);
    let distances = Arc::new(distances);

    for core_id in 0..1 {
        let name_indexes = name_indexes.clone();
        let names = names.clone();
        let flows = flows.clone();
        let neighbors = neighbors.clone();
        let distances = distances.clone();
        let best_score = best_score.clone();
        let corpus = corpus.clone();
        let orig_test = orig_test.clone();

        let t = std::thread::spawn(move || {
            run_simulation(
                core_id,
                name_indexes,
                names,
                flows,
                neighbors,
                distances,
                best_score,
                corpus,
                30,
                orig_test,
            )
        });

        threads.push(t);
    }

    for t in threads {
        let res = t.join();
    }

    /*
    macro_rules! spawn_threads {
        () => {{
            let corpus = Arc::new(Mutex::new(HashSet::new()));
            let mut threads = Vec::new();

            for core_id in 0..2 {
                let name_indexes = name_indexes.clone();
                let names = names.clone();
                let flows = flows.clone();
                let neighbors = neighbors.clone();
                let distances = distances.clone();
                let best_score = best_score.clone();
                let corpus = corpus.clone();
                let work = work.clone();

                let t = std::thread::spawn(move || {
                    while let Some((left, right)) = work.pop() {
                        run_simulation(
                            core_id,
                            name_indexes,
                            names,
                            flows,
                            neighbors,
                            distances,
                            best_score,
                            corpus,
                            26,
                            left,
                        );

                        let left_ans = best_score.load(Ordering::SeqCst);
                        best_score.store(0, Ordering::SeqCst);

                        let right_ans = run_simulation(
                            core_id,
                            name_indexes,
                            names,
                            flows,
                            neighbors,
                            distances,
                            best_score,
                            corpus,
                            26,
                            left,
                        );

                        let right_ans = best_score.load(Ordering::SeqCst);

                        let score = left_ans + right_ans;
                        best_score.fetch_max(score, Ordering::SeqCst);
                    }
                });

                threads.push(t);
            }

            for t in threads {
                let _ = t.join();
            }
        }};
    }
    */

    println!("Part 1 best score: {}", best_score.load(Ordering::SeqCst));

    //
    let mut best_score = 0;
    let mut work = Vec::new();
    for left in 1..valuables.len() {
        for curr_left in valuables.iter().combinations(left) {
            // Split the valuable nodes into two groups of unique nodes
            let curr_left = curr_left.iter().map(|x| **x).collect::<BTreeSet<_>>();

            let curr_right = valuables
                .difference(&curr_left)
                .copied()
                .collect::<BTreeSet<_>>();

            work.push((curr_left, curr_right));
        }
    }

    let work = Arc::new(Mutex::new(work));

    // for _ in 0..4 {
    // spawn_threads!();
    // }

    let mut threads = Vec::new();

    let orig_best_score = Arc::new(AtomicUsize::new(0));

    for core_id in 0..8 {
        let name_indexes = name_indexes.clone();
        let names = names.clone();
        let flows = flows.clone();
        let neighbors = neighbors.clone();
        let distances = distances.clone();
        let work = work.clone();
        let best_score = orig_best_score.clone();

        let t = std::thread::spawn(move || loop {
            let Some((left, right)) = work.lock().unwrap().pop() else { break; };

            let flows = flows.clone();
            let name_indexes = name_indexes.clone();
            let names = names.clone();
            let neighbors = neighbors.clone();
            let distances = distances.clone();
            let curr_score = Arc::new(AtomicUsize::new(0));
            let corpus = Arc::new(Mutex::new(HashSet::new()));

            run_simulation(
                core_id,
                name_indexes.clone(),
                names.clone(),
                flows.clone(),
                neighbors.clone(),
                distances.clone(),
                curr_score.clone(),
                corpus.clone(),
                26,
                left,
            );

            let corpus = Arc::new(Mutex::new(HashSet::new()));
            let left_ans = curr_score.load(Ordering::SeqCst);
            let curr_score = Arc::new(AtomicUsize::new(0));

            let curr_score = curr_score.clone();

            run_simulation(
                core_id,
                name_indexes,
                names,
                flows,
                neighbors,
                distances,
                curr_score.clone(),
                corpus,
                26,
                right,
            );

            let right_ans = curr_score.load(Ordering::SeqCst);

            let score = left_ans + right_ans;
            // println!("Score: {score} best: {}", best_score.load(Ordering::SeqCst));

            best_score.fetch_max(score, Ordering::SeqCst);
        });

        threads.push(t);
    }

    let work = work.clone();

    let t = std::thread::spawn(move || loop {
        let score = orig_best_score.load(Ordering::SeqCst);
        println!("Best: {score} Work left: {}", work.lock().unwrap().len());
        std::thread::sleep_ms(1000);
    });

    threads.push(t);

    for t in threads {
        t.join();
    }
}
