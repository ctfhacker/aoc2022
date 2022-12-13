#![feature(iter_next_chunk)]

const INPUT: &str = include_str!("../input");

/// A packet of data
/// 
/// A flat representation of the given packets for Day13
/// Each List or Value is inserted into the total `nodes` Vec.
/// 
/// 
/// Example:
/// [[1],[2,3,4]]
/// [
///   /* Index 0 */ [Index(1), Index(2), Value(255)]
///   /* Index 1 */ [Value(1), Value(255)], 
///   /* Index 2 */ [Value(2), Value(3), Value(4), Value(255)]
/// ]
///
/// When traversing the `nodes`, each `Index(index)` value points to `nodes[index]` for
/// the value that is represented in this location and each `Value(value)` is an actual value
/// at this location. The `255` is an indicator that this list is finished
/// 
/// [[1],4]
/// [
///   /* Index 0 */ [Index(1), Value(4), Value(255)], 
///   /* Index 1 */ [Value(1), Value(255)]
/// ], 
#[derive(Debug, Clone)]
struct Packet {
    // The nodes found for this packet
    nodes: Vec<Vec<Object>>,

    // Internal components for `.next`. 
    // If more perf is needed, pull these values out of the struct, pass them in by reference to
    // `.compare_packets` from local stack addresses
    // Don't really care that much about perf today ... >.<
    node_index: usize,
    curr_index: usize,
    stack: Vec<(usize, usize)>,

    /// Is this packet a decoder packet for Part2
    decoder_packet: bool
}

#[derive(Debug, Copy, Clone)]
enum Object {
    Value(u8),
    Index(usize),
}

impl Packet {
    /// Parse the blob manually from a str
    pub fn from_str(input: &[u8]) -> Packet {
        // The current node data is being inserted into
        let mut curr_node = 0;

        let mut nodes: Vec<Vec<Object>> = vec![];
        let mut stack = Vec::new();

        for section in input.split_inclusive(|x| *x == b'[' || *x == b']' || *x == b',') {
            match section {
                [b'['] => {
                    // Set the current node to the new node
                    curr_node = nodes.len();

                    // Initalize a new nodes vec
                    nodes.push(Vec::new());

                    // Add the current node to the stack in case there are inner vecs as well
                    stack.push(curr_node);
                }
                [b','] => {
                    // Continue
                }
                [b']'] => {
                    // Insert an `EMPTY_LIST` identifier for an empty list. 
                    // This is used to catch empty lists during the comparison
                    // of packets.
                    nodes.get_mut(curr_node).unwrap().push(Object::Value(255));

                    let _ = stack.pop().unwrap();
                    let Some(parent_node) = stack.iter().last() else { 
                        // With no more stack, we've reached the end
                        break; 
                    };

                    // Add the new node's index to the current node
                    nodes
                        .get_mut(*parent_node)
                        .unwrap()
                        .push(Object::Index(curr_node));

                    curr_node = *parent_node;
                }
                [val @ .., b','] => {
                    let val = std::str::from_utf8(val).unwrap().parse::<u8>().unwrap();
                    nodes.get_mut(curr_node).unwrap().push(Object::Value(val));
                }
                [val @ .., b']'] => {
                    let val = std::str::from_utf8(val).unwrap().parse::<u8>().unwrap();
                    nodes.get_mut(curr_node).unwrap().push(Object::Value(val));
                    nodes.get_mut(curr_node).unwrap().push(Object::Value(255));

                    let _ = stack.pop().unwrap();
                    let Some(parent_node) = stack.iter().last() else { 
                        // With no more stack, we've reached the end
                        break; 
                    };

                    // Add the new node's index to the current node
                    nodes
                        .get_mut(*parent_node)
                        .unwrap()
                        .push(Object::Index(curr_node));

                    curr_node = *parent_node;
                }
                _ => {}
            }
        }

        let decoder_packet = input == b"[[2]]" || input == b"[[6]]";

        Packet { nodes, curr_index: 0 , node_index: 0, stack: Vec::new(), decoder_packet }
    }

    fn next(&mut self) -> Option<Object> {
        if self.curr_index >= self.nodes[self.node_index].len() {
            let Some((old_node_index, old_curr_index)) = self.stack.pop() else {
                println!("Ran out of items!");
                return None;
            };

            self.node_index = old_node_index;
            self.curr_index = old_curr_index + 1;
        }

        let res = self.nodes.get(self.node_index).and_then(|x| x.get(self.curr_index))?;

        match res {
            Object::Index(index) => {
                self.stack.push((self.node_index, self.curr_index));
                self.node_index = *index;
                self.curr_index = 0;
            }
            Object::Value(_) => {
                self.curr_index += 1;
            }
        }

        Some(*res)
    }
}

/// Returns `true` if the left/right ordering is correct and `false` otherwise
fn compare_packets(left: &mut Packet, right: &mut Packet) -> bool {
    for _iter in 0.. {
        let left_val = left.next();
        let right_val = right.next();

        match (left_val, right_val) {
            (None, None) => {
                panic!("Both ran out?!");
            }
            (Some(Object::Index(_)), Some(Object::Index(_))) => {
                // Two indexes both need traversing. Keep traversing
            }
            (Some(Object::Value(l)), Some(Object::Value(r))) => {
                // Found two values, check if they signal a comparison result

                // Two end of lists can continue iterating
                if l == 255 && r == 255 {
                    continue;
                }

                // If left is an end and right is not
                if l == 255 {
                    return true;
                }
                // If right is an end and left is not
                if r == 255 {
                    return false;
                }

                // Normal comparison rules from the problem
                if l < r {
                    return true;
                }
                if r < l {
                    return false;
                }
            }
            // Left ran out of items
            (None, Some(_)) => {
                return true;
            }
            // Right ran out of items
            (Some(_), None) => {
                return false;
            }
            // Left ran out of items
            (Some(Object::Value(255)), Some(Object::Index(_))) => {
                return true;
            }
            // Right ran out of items
            (Some(Object::Index(_)), Some(Object::Value(255))) => {
                return false;
            }
            (Some(Object::Value(l)), Some(Object::Index(_))) => {
                // Left has a value and right is still pointing to a list
                // Continue traversing the lists until a value has been reached
                let mut right_val2 = right.next();
                loop {
                    if matches!(right_val2, Some(Object::Value(_))) {
                        break;
                    }

                    right_val2 = right.next();
                }

                // Perform the normal comparison of two values
                match right_val2 {
                    Some(Object::Value(r2)) => {
                        if l == 255 && r2 == 255 {
                            panic!();
                        }

                        if l == 255 {
                            return true;
                        }
                        if r2 == 255 {
                            return false;
                        }
                        if l < r2 {
                            return true;
                        }
                        if r2 < l {
                            return false;
                        }
                        if l == r2 {
                            return true;
                        }
                    }
                    Some(Object::Index(_)) => {
                        // Left ran out of items
                        return true;
                    }
                    None => {
                        return false;
                    }
                }
            }
            (Some(Object::Index(_)), Some(Object::Value(r))) => {
                // Right has a value and left is still pointing to a list
                // Continue traversing the lists until a value has been reached
                let mut left_val2 = left.next();
                loop {
                    if matches!(left_val2, Some(Object::Value(_))) {
                        break;
                    }

                    left_val2 = left.next();
                }

                // Perform the normal comparison of two values
                match left_val2 {
                    Some(Object::Value(l2)) => {
                        if l2 == 255 && r == 255 {
                            panic!("Two end of lists?!");
                        }
                        if l2 == 255 {
                            return true;
                        }
                        if r == 255 {
                            return false;
                        }
                        if l2 < r {
                            return true;
                        }
                        if r < l2 {
                            return false;
                        }

                        if r == l2 {
                            // Left will run out of items
                            return false;
                        }

                    }
                    Some(Object::Index(_)) => {
                        // Right ran out of items
                        return false;
                    }
                    None => {
                        return true;
                    }
                }
            }
        }
    }

    panic!("Unknown result value found");
}

fn main() {
    let mut sum = 0;

    for (index, section) in INPUT.split("\n\n").enumerate() {
        let index = index + 1;
        if let Ok([left_line, right_line]) = section
            .split('\n')
            .take(2)
            .map(|line| line.as_bytes())
            .next_chunk()
        {
            let mut left = Packet::from_str(left_line);
            let mut right = Packet::from_str(right_line);

            if compare_packets(&mut left, &mut right) {
                sum += index;
            }
        }
    }

    println!("Part 1 SUM:  {sum}");

    // Parse the input into individual lines
    let mut data = INPUT.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Packet::from_str(line.as_bytes()))
        .collect::<Vec<_>>();

    // Add the decoder packets to the data
    data.push(Packet::from_str(b"[[2]]"));
    data.push(Packet::from_str(b"[[6]]"));

    // Sort the data based on the compare_packets function written in Part 1
    // Sadly, these `clones` are going to be slow since the Packet internal traversal needs a `&mut Packet`. 
    data.sort_by(|a, b| {
        let mut a = a.clone();
        let mut b = b.clone();
        match compare_packets(&mut a, &mut b) {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        }
    });

    // Find the decoder packets and multiply their indexes for the result
    let mut result = 1;
    for (index, _line) in data.iter().enumerate().filter(|(_i, p)| p.decoder_packet) {
        result *= index + 1;
    }

    println!("Part 2 PROD: {result}");
}
