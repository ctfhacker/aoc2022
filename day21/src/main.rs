#![feature(iter_next_chunk)]
const INPUT: &str = include_str!("../input");

use z3::ast::Ast;
use z3::*;

use std::collections::HashMap;

fn main() {
    // Initialize Z3's context and solver
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Initialize the database of variables used by z3
    let mut vars = HashMap::new();

    for part in [1, 2] {
        for line in INPUT.split('\n').filter(|x| !x.is_empty()) {
            // Add the current equation's destination variable into Z3 if it does not already exist
            let [dest, src] = line.split(": ").next_chunk().unwrap();
            vars.entry(dest).or_insert(ast::Int::new_const(&ctx, dest));

            // Attempt to parse the first variable of each equation.
            // If first arg parses:        <dest> = <Literal>
            // If first arg doesn't parse: <dest> = <var1> <op> <var2>
            let mut src_iter = src.split(" ");
            let left = src_iter.next().unwrap();
            match left.parse::<i64>() {
                Ok(val) => {
                    // For part 2, ignore the original humn value
                    if part == 2 && dest == "humn" {
                        continue;
                    }

                    // Raw value assignment. Set this destination as the value
                    let val = ast::Int::from_i64(&ctx, val);
                    let dest = &vars[dest];
                    solver.assert(&dest._eq(&val));
                }
                Err(_) => {
                    // If there isn't a raw value in the left argument, the equation is
                    // of type var1 <op> var2
                    let mut op = src_iter.next().unwrap();
                    let right = src_iter.next().unwrap();

                    // Insert each operand for this equation if it doesn't already exist
                    vars.entry(left).or_insert(ast::Int::new_const(&ctx, left));
                    vars.entry(right)
                        .or_insert(ast::Int::new_const(&ctx, right));

                    // Save the destination name to check for `root` for part2
                    let dest_name = dest;

                    // Get the operands and destination
                    let left = &vars[left];
                    let right = &vars[right];
                    let dest = &vars[dest];

                    // Part 2 calls for the `root` equation to change to equality
                    if part == 2 && dest_name == "root" {
                        op = "=";
                    }

                    match op {
                        "+" => {
                            // Add an addition condition to the solver
                            let equation = ast::Int::add(&ctx, &[&left, &right]);
                            solver.assert(&dest._eq(&equation));
                        }
                        "-" => {
                            // Add a subtraction condition to the solver
                            let equation = ast::Int::sub(&ctx, &[&left, &right]);
                            solver.assert(&dest._eq(&equation));
                        }
                        "*" => {
                            // Add a multiply condition to the solver
                            let equation = ast::Int::mul(&ctx, &[&left, &right]);
                            solver.assert(&dest._eq(&equation));
                        }
                        "/" => {
                            // Add a divide condition to the solver
                            let equation = ast::Int::div(&left, &right);
                            solver.assert(&dest._eq(&equation));
                        }
                        "=" => {
                            // Add an equality condition to the solver
                            solver.assert(&left._eq(&right));
                        }
                        _ => panic!(),
                    }
                }
            }
        }

        // Snaity check the model is satisfiable
        assert_eq!(solver.check(), SatResult::Sat);

        // Get a satisfiable model
        let model = solver.get_model().unwrap();

        if part == 1 {
            // Get the value for `root` that satisfies the series of equations
            let root = model.eval(&vars["root"], true).unwrap().as_i64().unwrap();
            println!("Part 1: {root}");
        } else if part == 2 {
            // Get the value for `humn` that satisfies the series of equations
            let humn = model.eval(&vars["humn"], true).unwrap().as_i64().unwrap();
            println!("Part 2: {humn}");
        }

        // Reset the solver for part 2
        solver.reset();
    }
}
