use std::collections::VecDeque;
use std::ops::Add;
use std::ops::Mul;

const INPUT: &str = include_str!("../input");

#[derive(Debug)]
enum Day11Error {
    Num(std::num::ParseIntError),
    UnknownOperation(&'static str),
}

/// The operation to perform
#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Mul,
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    /// Items currently held by the monkey
    items: VecDeque<u64>,

    /// Number of items this monkey has inspected
    inspected: u64,

    /// Operation used to change the items being moved
    operation: Operation,

    /// The right hand side of the operation
    operation_val: u64,

    /// `divisible by` value
    test_val: u64,

    /// Monkey ID to throw the item true on `true` result of `item % test_val == 0`
    true_monkey: usize,

    /// Monkey ID to throw the item false on `false` result of `item % test_val == 0`
    false_monkey: usize,
}

impl Monkey {
    pub fn from_str(data: &'static str) -> Result<Monkey, Day11Error> {
        let mut iter = data.lines();

        // Ignore the monkey ID
        let _monkey_id = iter.next();

        // Parse line 2: Monkey items
        // Expected input:
        // Starting items: 60, 84, 84, 65
        let items_iter = iter.next().unwrap().split(":").skip(1).next().unwrap();
        let mut items = VecDeque::new();

        for item in items_iter.split(" ").filter(|x| !x.is_empty()) {
            println!("{item}");
            let item = item
                .replace(",", "")
                .parse::<u64>()
                .map_err(|e| Day11Error::Num(e))?;

            items.push_back(item);
        }

        let op_line = iter.next().unwrap();

        let mut op_iter = op_line
            .split(" new = old ")
            .skip(1)
            .next()
            .unwrap()
            .split(" ");

        let mut op = match op_iter.next() {
            Some("+") => Operation::Add,
            Some("*") => Operation::Mul,
            _ => return Err(Day11Error::UnknownOperation(op_line)),
        };

        let op_val = match op_iter.next() {
            Some("old") => {
                op = Operation::Square;
                0
            }
            Some(val) => val.parse::<u64>().map_err(|e| Day11Error::Num(e))?,
            _ => return Err(Day11Error::UnknownOperation(op_line)),
        };

        let test_val = iter
            .next()
            .unwrap()
            .split("divisible by ")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .map_err(|e| Day11Error::Num(e))?;

        let true_monkey = iter
            .next()
            .unwrap()
            .split("If true: throw to monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .map_err(|e| Day11Error::Num(e))?;

        let false_monkey = iter
            .next()
            .unwrap()
            .split("If false: throw to monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .map_err(|e| Day11Error::Num(e))?;

        Ok(Monkey {
            items,
            inspected: 0,
            operation: op,
            operation_val: op_val,
            test_val,
            true_monkey,
            false_monkey,
        })
    }
}

fn main() -> Result<(), Day11Error> {
    let mut orig_monkeys = Vec::new();

    fn operation(op: Operation, old: u64, val: u64) -> u64 {
        match op {
            Operation::Add => old + val,
            Operation::Mul => old * val,
            Operation::Square => old * old,
            _ => unreachable!(),
        }
    }

    // Expected input:
    // Monkey 0:
    //   Starting items: 64
    //   Operation: new = old * 7
    //   Test: divisible by 13
    //     If true: throw to monkey 1
    //     If false: throw to monkey 3
    for section in INPUT.split("\n\n") {
        orig_monkeys.push(Monkey::from_str(section)?);
    }

    for part in [Some(3), None] {
        // Reset the monkeys for each part
        let mut monkeys = orig_monkeys.clone();

        // Get the least common multiple for all of the test values
        let div_by: u64 = monkeys.iter().map(|m| m.test_val).product();

        for round in 1..10001 {
            for index in 0..monkeys.len() {
                loop {
                    let Some(item) = monkeys[index].items.pop_front() else { break };
                    let monkey = &mut monkeys[index];

                    // Increment the number of items this monkey has inspected
                    monkey.inspected += 1;

                    // Calculate the new value of the item based on the operation and value for this Monkey
                    let mut val = operation(monkey.operation, item, monkey.operation_val);

                    // Part 1 had a divide by 3 rule for the items
                    if let Some(div_val) = part {
                        val /= div_val;
                    } else {
                        val %= div_by;
                    }

                    // Based on the result of the new value, move the item to another monkey
                    if val % monkey.test_val == 0 {
                        let true_monkey = monkey.true_monkey;
                        monkeys[true_monkey].items.push_back(val);
                    } else {
                        let false_monkey = monkey.false_monkey;
                        monkeys[false_monkey].items.push_back(val);
                    }
                }
            }

            if [1, 20, 1000, 2000, 10000].contains(&round) {
                let mut best = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();

                let check = match round {
                    1 => [2, 4, 3, 6],
                    20 => [99, 97, 8, 103],
                    1000 => [5204, 4792, 199, 5192],
                    _ => [0, 0, 0, 0],
                };
                if best == check {
                    println!("{round} | Div by: {div_by}")
                }

                println!("{best:?}");

                best.sort();
                best.reverse();
                if let Some([one, two]) = best.chunks(2).next() {
                    println!("{one} {two} {}", one * two);
                }
            }
        }
    }

    Ok(())
}
