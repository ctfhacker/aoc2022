const INPUT: &str = include_str!("../input");

/// Errors possible during Day 10
#[derive(Debug, Clone)]
enum Day10Error {
    /// An instruction was not a valid format: <OPCODE> [<VALUE>]
    InvalidInstructionFormat(String),

    /// A parsed instruction value did not fit in an `i32`
    InvalidArgument(String),

    /// Parsed an unknown opcode
    UnknownOpcode(String),

    /// Attempted to execute without loading an instruction first
    InstructionNotLoaded,
}

/// The execution unit of the processor for Day 10
struct Cpu {
    /// The current instructions in the CPU
    instructions: Vec<Instruction>,

    /// The register state of the CPU
    registers: [i32; Register::Count as usize],

    /// The currently executing instruction if it takes longer than 1 clock cycle
    pipeline: Option<Pipeline>,

    /// The instruction pointer
    ip: usize,

    /// Number of cycles executed
    cycles_executed: usize,
}

/// Is the CPU continuing to execute or is it finished
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Execution {
    Continue,
    Finished,
}

impl Cpu {
    pub fn from_input(input: &str) -> Result<Self, Day10Error> {
        let mut instructions = Vec::new();

        // Parse each instruction line
        for line in INPUT.split("\n") {
            if line.is_empty() {
                continue;
            }

            instructions.push(Instruction::try_from(line)?);
        }

        // Init the register state
        let mut registers = [0; Register::Count as usize];
        registers[Register::X as usize] = 1;

        Ok(Cpu {
            instructions,
            registers,
            pipeline: None,
            ip: 0,

            // The problem starts cycles at 1. Use this value here to calculate the
            // correct cycles when checking for an answer
            cycles_executed: 1,
        })
    }

    /// Apply a given instruction to the current CPU state
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {
                // Nothing to do
            }
            Instruction::Addx(val) => {
                self.registers[Register::X as usize] += val;
            }
        }
    }

    /// Step the CPU one clock cycle
    pub fn step(&mut self) -> Result<Execution, Day10Error> {
        // Grab the next instruction if there isn't one already executing
        if self.pipeline.is_none() {
            // If there are no more instructions to execute, the CPU is finished
            if self.ip >= self.instructions.len() {
                return Ok(Execution::Finished);
            }

            // Fetch the next instruction
            let instr = self.instructions[self.ip];

            // Increment the instruction pointer
            self.ip += 1;

            // Set the instruction into the pipeline
            self.pipeline = Some(Pipeline {
                instruction: instr,
                cycles_left: instr.cycles(),
            });
        }

        // Each step call will always step the CPU cycles
        self.cycles_executed += 1;

        // Check if there is already an existing instruction executing in this pipeline
        let Some(Pipeline { instruction, mut cycles_left }) = self.pipeline.take() else {
            return Err(Day10Error::InstructionNotLoaded);
        };

        // Reduce the number of cycles left for this instruction by 1
        cycles_left -= 1;

        // If this instruction has finished, set the instruction to execute as this
        // one and reset the pipeline
        if cycles_left == 0 {
            self.execute(instruction);
        } else {
            // There are still cycles left to execute this instruction, so we can't
            // execute it yet. Set the pipeline back and continue.
            self.pipeline = Some(Pipeline {
                instruction,
                cycles_left,
            });
        }

        // CPU still has instructions to execute, continue
        Ok(Execution::Continue)
    }

    /// Print the CPU state
    pub fn print(&self) {
        println!("------- Cycle {:03} -------", self.cycles_executed);
        println!(" X: {:4}", self.registers[Register::X as usize]);
        println!("IP: {:4}", self.ip);
        println!("--- Pipeline ---");
        println!("{:?}", self.pipeline);
        println!("--- Instrs ---");
        for ip in self.ip..(self.ip + 5).min(self.instructions.len()) {
            println!("{ip:4}: {:?}", self.instructions[ip]);
        }
        println!();
    }
}

/// A particular pipeline in the CPU that can execute opcodes for a given number of
/// instructions
#[derive(Debug, Copy, Clone)]
struct Pipeline {
    instruction: Instruction,
    cycles_left: u32,
}

/// Registers available in the CPU
enum Register {
    X,

    // Used to count the number of enum variants
    Count,
}

/// Instructions available for our CPU
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    pub fn cycles(self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = Day10Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut iter = line.split(" ");

        // Parse the opcode from the instruction
        let opcode = iter
            .next()
            .ok_or_else(|| Day10Error::InvalidInstructionFormat(line.to_string()))?;

        // Get the next argument for an instruction
        macro_rules! arg {
            () => {
                iter.next()
                    .ok_or_else(|| Day10Error::InvalidInstructionFormat(line.to_string()))?
                    .parse::<i32>()
                    .map_err(|_| Day10Error::InvalidArgument(line.to_string()))?
            };
        }

        match opcode {
            "addx" => Ok(Instruction::Addx(arg!())),
            "noop" => Ok(Instruction::Noop),
            _ => Err(Day10Error::UnknownOpcode(opcode.to_string())),
        }
    }
}

fn main() -> Result<(), Day10Error> {
    let mut cpu = Cpu::from_input(&INPUT)?;
    let mut execution = Execution::Continue;

    let mut sum = 0;

    let mut display = [['.'; 40]; 6];

    while execution != Execution::Finished {
        // Get the current cycles executed before stepping the CPU
        let cycles = cpu.cycles_executed as i32;

        // Only 240 cycles for this problem
        if cycles >= 240 {
            break;
        }

        // Write the CRT pixel to the display if it is in bounds of the x position of the
        // sprite
        let x_pos = cpu.registers[Register::X as usize];
        let col = ((cycles - 1) % 40) as usize;
        let row = ((cycles - 1) / 40) as usize;
        if [x_pos - 1, x_pos, x_pos + 1].contains(&(col as i32)) {
            display[row][col] = '#';
        } else {
            display[row][col] = '.';
        }

        // Step the CPU
        execution = cpu.step()?;

        // Get the new cycles executed
        let cycles = cpu.cycles_executed as i32;
        let x_pos = cpu.registers[Register::X as usize];

        // Calculate the signal strengths for part 1
        if [20, 60, 100, 140, 180, 220].contains(&cycles) {
            sum += cycles as i32 * cpu.registers[Register::X as usize];
        }
    }

    println!("Part 1 signal strengths: {sum}");

    for line in display {
        for c in line {
            print!("{c}");
        }
        println!();
    }

    Ok(())
}
