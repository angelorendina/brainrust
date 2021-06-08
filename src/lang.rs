use std::collections::VecDeque;

/// Executable instruction for the VM.
///
/// Jump and Loop include an indication of what the IP should be set to, if jumping.
#[derive(Clone, Copy, PartialEq)]
enum Instruction {
    Right,
    Left,
    Up,
    Down,
    Out,
    In,
    Jump(usize),
    Loop(usize),
    None,
}

impl Instruction {
    /// Source character to instruction.
    ///
    /// All chars other than ><+-[] are ignored.
    fn from_char(c: char) -> Self {
        match c {
            '>' => Instruction::Right,
            '<' => Instruction::Left,
            '+' => Instruction::Up,
            '-' => Instruction::Down,
            '.' => Instruction::Out,
            ',' => Instruction::In,
            '[' => Instruction::Jump(0),
            ']' => Instruction::Loop(0),
            _ => Instruction::None,
        }
    }
}

/// Returned when compilation fails.
pub struct SyntaxError;

/// Virtual machine.
pub struct VM {
    ip: usize,
    dp: usize,
    instructions: Vec<Instruction>,
    data: VecDeque<u8>,
}

impl VM {
    /// Constructs a new valid VM from given source code.
    ///
    /// Compilation fails if brackets are not properly paired (invalid program); a SyntaxError gets returned if so.
    pub fn construct(src: &str) -> Result<Self, SyntaxError> {
        let mut instructions: Vec<Instruction> = Vec::with_capacity(src.len());
        let mut jumps: Vec<usize> = Vec::with_capacity(src.len() / 2);

        // Converts source code into vector of Instructions.
        // Caches position of each Jump in a FILO stack, that gets popped at each corresponding Loop.
        for c in src.chars() {
            let instr = Instruction::from_char(c);
            if instr != Instruction::None {
                if let Instruction::Loop(_) = instr {
                    if let Some(loop_to) = jumps.pop() {
                        instructions[loop_to] = Instruction::Jump(instructions.len());
                        instructions.push(Instruction::Loop(loop_to));
                    } else {
                        return Err(SyntaxError);
                    }
                }
                if let Instruction::Jump(_) = instr {
                    jumps.push(instructions.len());
                }
                instructions.push(instr);
            }
        }

        // Syntax error if not all Jumps have matching Loop.
        if jumps.len() > 0 {
            return Err(SyntaxError);
        }

        // Appends None. Used to halt the VM.
        instructions.push(Instruction::None);

        // Code is successfully compiled.
        return Ok(Self {
            ip: 0,
            dp: 0,
            instructions,
            data: VecDeque::from(vec![0]),
        });
    }

    /// Performs one step for the VM. It is assumed to be in a valid state.
    ///
    /// Returns True if None was reached, and False otherwise.
    pub fn step(&mut self, input: &mut VecDeque<u8>, output: &mut Vec<u8>) -> bool {
        match self.instructions[self.ip] {
            Instruction::Right => {
                self.dp += 1;
                if self.data.len() == self.dp {
                    self.data.push_back(0);
                }
            }
            Instruction::Left => {
                if self.dp > 0 {
                    self.dp -= 1;
                } else {
                    self.data.push_front(0);
                }
            }
            Instruction::Up => {
                if self.data[self.dp] == u8::MAX {
                    self.data[self.dp] = 0;
                } else {
                    self.data[self.dp] += 1;
                }
            }
            Instruction::Down => {
                if self.data[self.dp] == 0 {
                    self.data[self.dp] = u8::MAX;
                } else {
                    self.data[self.dp] -= 1;
                }
            }
            Instruction::Out => {
                output.push(self.data[self.dp]);
            }
            Instruction::In => {
                if let Some(b) = input.pop_front() {
                    self.data[self.dp] = b;
                } else {
                    self.data[self.dp] = 0;
                }
            }
            Instruction::Jump(dest) => {
                let do_jump = self.data[self.dp] == 0;
                if do_jump {
                    self.ip = dest;
                }
            }
            Instruction::Loop(dest) => {
                let do_loop = self.data[self.dp] != 0;
                if do_loop {
                    self.ip = dest;
                }
            }
            Instruction::None => {
                return true;
            }
        }
        self.ip += 1;
        return self.instructions[self.ip as usize] == Instruction::None;
    }

    /// Runs the VM to None. It is assumed to be in a valid state.
    pub fn run(&mut self, input: &mut VecDeque<u8>, output: &mut Vec<u8>) {
        while !self.step(input, output) {}
    }
}
