use itertools::Itertools;
use std::str::FromStr;
use utils::solution::Solution;

pub struct Day23 {}

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> Option<String> {
        let instructions = input.lines().flat_map(Instruction::from_str).collect_vec();
        let mut machine = Machine::new((0, 0), instructions);

        for _ in machine.by_ref() {}

        Some(machine.register_b.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let instructions = input.lines().flat_map(Instruction::from_str).collect_vec();
        let mut machine = Machine::new((1, 0), instructions);

        for _ in machine.by_ref() {}

        Some(machine.register_b.to_string())
    }
}

type Offset = i32;
type RegisterValue = usize;

#[derive(Clone, Copy, Debug)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Self::A,
            "b" => Self::B,
            _ => Err("Invalid register")?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Half(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
    Triple(Register),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        Ok(match tokens.next() {
            None => Err("Empty instruction")?,
            Some("hlf") => {
                Self::Half(Register::from_str(tokens.next().ok_or("Bad token get")?).unwrap())
            }
            Some("inc") => {
                Self::Increment(Register::from_str(tokens.next().ok_or("Bad token get")?).unwrap())
            }
            Some("jmp") => Self::Jump(tokens.next().ok_or("Bad token get")?.parse().unwrap()),
            Some("jie") => {
                if let Some(reg) = tokens.next() {
                    Self::JumpIfEven(
                        Register::from_str(reg.trim_end_matches(',')).unwrap(),
                        tokens.next().ok_or("Bad token get")?.parse().unwrap(),
                    )
                } else {
                    Err("Invalid jie arguments")?
                }
            }
            Some("jio") => {
                if let Some(reg) = tokens.next() {
                    Self::JumpIfOne(
                        Register::from_str(reg.trim_end_matches(',')).unwrap(),
                        tokens.next().ok_or("Bad token get")?.parse().unwrap(),
                    )
                } else {
                    Err("Invalid jio arguments")?
                }
            }
            Some("tpl") => {
                Self::Triple(Register::from_str(tokens.next().ok_or("Bad token get")?).unwrap())
            }
            Some(_) => Err("Unknown instruction")?,
        })
    }
}

#[derive(Clone, Debug)]
struct Machine {
    instructions: Vec<Instruction>,
    instruction_ptr: usize,
    register_a: RegisterValue,
    register_b: RegisterValue,
}

impl Machine {
    fn new((a, b): (RegisterValue, RegisterValue), instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            instruction_ptr: 0,
            register_a: a,
            register_b: b,
        }
    }

    fn get_register(&self, register: Register) -> RegisterValue {
        match register {
            Register::A => self.register_a,
            Register::B => self.register_b,
        }
    }

    fn get_register_mut(&mut self, register: Register) -> &mut RegisterValue {
        match register {
            Register::A => &mut self.register_a,
            Register::B => &mut self.register_b,
        }
    }
}

impl Iterator for Machine {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(instruction) = self.instructions.get(self.instruction_ptr) {
            match *instruction {
                Instruction::Half(reg) => {
                    *self.get_register_mut(reg) /= 2;
                    self.instruction_ptr += 1;
                }
                Instruction::Increment(reg) => {
                    *self.get_register_mut(reg) += 1;
                    self.instruction_ptr += 1;
                }
                Instruction::Jump(value) => {
                    self.instruction_ptr = (self.instruction_ptr as i32 + value) as usize
                }
                Instruction::JumpIfEven(reg, value) => {
                    if self.get_register(reg) % 2 == 0 {
                        self.instruction_ptr = (self.instruction_ptr as i32 + value) as usize;
                    } else {
                        self.instruction_ptr += 1;
                    }
                }
                Instruction::JumpIfOne(reg, value) => {
                    if self.get_register(reg) == 1 {
                        self.instruction_ptr = (self.instruction_ptr as i32 + value) as usize;
                    } else {
                        self.instruction_ptr += 1
                    }
                }
                Instruction::Triple(reg) => {
                    *self.get_register_mut(reg) *= 3;
                    self.instruction_ptr += 1;
                }
            }

            Some(())
        } else {
            None
        }
    }
}
