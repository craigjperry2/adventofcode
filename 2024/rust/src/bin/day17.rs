use itertools::Itertools;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let mut computer: Computer = include_str!("../../../data/day17.txt")
        .parse()
        .expect("Failed to parse computer");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let result = part1(&mut computer);
    println!(
        "Part 1: '{result}' took {}µs",
        sw_part1.elapsed().as_micros()
    );
}

// ------------------- PART 1 --------------------

fn part1(computer: &mut Computer) -> String {
    println!("{computer}");
    computer.execute()
}

// ------------------- TYPES: COMPUTER --------------------

struct Computer {
    register_a: i32,
    register_b: i32,
    register_c: i32,
    ip: usize,
    program: Vec<Instruction>,
}

impl Computer {
    fn execute(&mut self) -> String {
        let denominator: i32 = 2;
        let mut stdout: Vec<String> = Vec::new();
        while self.ip < self.program.len() * 2 {
            match &self.program[self.ip / 2] {
                Instruction::Adv(op) => {
                    // The adv instruction (opcode 0) performs division. The numerator is the value in
                    // the A register. The denominator is found by raising 2 to the power of the instruction's
                    // combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would
                    // divide A by 2^B.) The result of the division operation is truncated to an integer and
                    // then written to the A register.
                    self.register_a /= denominator.pow(u32::try_from(op.value(self)).unwrap());
                    self.ip += 2;
                }
                Instruction::Bxl(Literal::Operand(op)) => {
                    // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
                    // instruction's literal operand, then stores the result in register B.
                    self.register_b ^= *op;
                    self.ip += 2;
                }
                Instruction::Bst(op) => {
                    // The bst instruction (opcode 2) calculates the value of its combo operand modulo
                    // 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                    self.register_b = op.value(self) & 0b111;
                    self.ip += 2;
                }
                Instruction::Jnz(Literal::Operand(op)) => {
                    // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if
                    // the A register is not zero, it jumps by setting the instruction pointer to the
                    // value of its literal operand; if this instruction jumps, the instruction pointer
                    // is not increased by 2 after this instruction.
                    if self.register_a != 0 {
                        self.ip = *op as usize;
                        println!("Jumping to {}", self.ip);
                    } else {
                        self.ip += 2;
                    }
                }
                Instruction::Bxc => {
                    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register
                    // C, then stores the result in register B. (For legacy reasons, this instruction reads
                    // an operand but ignores it.)
                    self.register_b ^= self.register_c;
                    self.ip += 2;
                }
                Instruction::Out(op) => {
                    // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
                    // then outputs that value. (If a program outputs multiple values, they are separated
                    // by commas.)
                    let result = op.value(self) % 8;
                    stdout.push(result.to_string());
                    self.ip += 2;
                }
                Instruction::Bdv(op) => {
                    // The bdv instruction (opcode 6) works exactly like the adv instruction except that
                    // the result is stored in the B register. (The numerator is still read from the A
                    // register.)
                    self.register_b = self.register_a / denominator.pow(u32::try_from(op.value(self)).unwrap());
                    self.ip += 2;
                }
                Instruction::Cdv(op) => {
                    // The cdv instruction (opcode 7) works exactly like the adv instruction except that
                    // the result is stored in the C register. (The numerator is still read from the A
                    // register.)
                    self.register_c = self.register_a / denominator.pow(u32::try_from(op.value(self)).unwrap());
                    self.ip += 2;
                }
            }
            println!("{}", self);
        }
        stdout.join(",")
    }
}

impl FromStr for Computer {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, program) = s.split_once("\n\n").expect("Failed to parse state dump");
        let mut cs = state.lines().map(|l| l[12..].parse::<i32>().unwrap());

        Ok(Self {
            register_a: cs.next().unwrap(),
            register_b: cs.next().unwrap(),
            register_c: cs.next().unwrap(),
            ip: 0,
            program: program
                .split(&[' ', ','])
                .skip(1)
                .tuples()
                .map(|(opcode, operand)| [opcode, operand].join(",").parse().unwrap())
                .collect(),
        })
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
                f,
                "Computer\n--------\nRegister A: {}\nRegister B: {}\nRegister C: {}\nInstruction Pointer: {}\nProgram:\n{}{}\n",
                self.register_a, self.register_b, self.register_c, self.ip, self
                    .program
                    .iter()
                    .enumerate()
                    .map(|(i, instr)| format!("{} {i}: {instr}", if i == self.ip / 2 { "IP>" } else { "   " }))
                    .join("\n"), if self.ip == self.program.len() * 2 { "\nIP> -: Program complete" } else { "" }
            )
    }
}

// ------------------- TYPES: INSTRUCTION --------------------

enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, operand) = s.split_once(',').unwrap();
        match opcode {
            "0" => Ok(Self::Adv(Combo::Operand(operand.parse().unwrap()))),
            "1" => Ok(Self::Bxl(Literal::Operand(operand.parse().unwrap()))),
            "2" => Ok(Self::Bst(Combo::Operand(operand.parse().unwrap()))),
            "3" => Ok(Self::Jnz(Literal::Operand(operand.parse().unwrap()))),
            "4" => Ok(Self::Bxc),
            "5" => Ok(Self::Out(Combo::Operand(operand.parse().unwrap()))),
            "6" => Ok(Self::Bdv(Combo::Operand(operand.parse().unwrap()))),
            "7" => Ok(Self::Cdv(Combo::Operand(operand.parse().unwrap()))),
            _ => panic!("Unknown instruction: {s}"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Instruction::Adv(op) => write!(f, "Adv({op})"),
            Instruction::Bxl(op) => write!(f, "Bxl({op})"),
            Instruction::Bst(op) => write!(f, "Bst({op})"),
            Instruction::Jnz(op) => write!(f, "Jnz({op})"),
            Instruction::Bxc => write!(f, "Bxc"),
            Instruction::Out(op) => write!(f, "Out({op})"),
            Instruction::Bdv(op) => write!(f, "Bdv({op})"),
            Instruction::Cdv(op) => write!(f, "Cdv({op})"),
        }
    }
}

// ------------------- TYPES: OPERANDS --------------------

enum Combo {
    Operand(i32),
}

impl Combo {
    fn value(&self, computer: &Computer) -> i32 {
        match self {
            Combo::Operand(op) if *op == 4 => computer.register_a,
            Combo::Operand(op) if *op == 5 => computer.register_b,
            Combo::Operand(op) if *op == 6 => computer.register_c,
            Combo::Operand(op) if *op == 7 => panic!("Operand 7 is reserved"),
            Combo::Operand(op) => *op,
        }
    }
}

impl Display for Combo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Combo::Operand(op) if *op == 4 => write!(f, "register_a"),
            Combo::Operand(op) if *op == 5 => write!(f, "register_b"),
            Combo::Operand(op) if *op == 6 => write!(f, "register_c"),
            Combo::Operand(op) => write!(f, "{}", op),
        }
    }
}

enum Literal {
    Operand(i32),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Literal::Operand(op) => write!(f, "{}", op),
        }
    }
}
