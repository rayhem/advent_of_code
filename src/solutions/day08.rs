use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day08 {}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> Option<String> {
        let instructions = input
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect::<Tape>();

        Some(VirtualMachine::new(instructions).run().acc.to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

type Tape = Vec<Instruction>;

#[derive(Clone, Copy, Debug)]
enum Day08Error {
    BadCommand,
    UnknownCommand,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

impl std::str::FromStr for Instruction {
    type Err = Day08Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, arg) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or(Day08Error::BadCommand)?;

        match cmd {
            "acc" => Ok(Instruction::Acc(arg.parse().unwrap())),
            "jmp" => Ok(Instruction::Jmp(arg.parse().unwrap())),
            "nop" => Ok(Instruction::Nop),
            _ => Err(Day08Error::UnknownCommand),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    NotStarted,
    Executing,
    InfiniteLoop,
    Terminated,
}

#[derive(Clone, Debug)]
struct VirtualMachine {
    instructions: Tape,
    hit_count: Vec<i32>,
    status: Status,

    pub ip: i32,
    pub acc: i32,
}

impl VirtualMachine {
    fn new(instructions: Vec<Instruction>) -> Self {
        let n = instructions.len();
        VirtualMachine {
            instructions: instructions,
            hit_count: vec![0; n],
            status: Status::NotStarted,
            ip: 0,
            acc: 0,
        }
    }

    fn run(&mut self) -> &Self {
        while self.status != Status::Terminated && self.status != Status::InfiniteLoop {
            self.step();
        }

        self
    }

    fn step(&mut self) -> &Self {
        self.hit_count[self.ip as usize] += 1;
        self.set_status();

        if self.status == Status::Executing {
            self.run_current_instruction()
        }

        self
    }

    fn set_status(&mut self) {
        let ip = self.ip as usize;

        self.status = if ip == self.instructions.len() {
            Status::Terminated
        } else {
            match self.hit_count[ip] {
                i if i <= 1 => Status::Executing,
                _ => Status::InfiniteLoop,
            }
        };
    }

    fn run_current_instruction(&mut self) {
        match self.instructions[self.ip as usize] {
            Instruction::Acc(n) => {
                self.acc += n;
                self.ip += 1
            }
            Instruction::Jmp(n) => self.ip += n,
            Instruction::Nop => self.ip += 1,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parse() {
        use Instruction::*;
        let inputs = vec![
            ("nop +0", Nop),
            ("acc +1", Acc(1)),
            ("jmp +4", Jmp(4)),
            ("acc +3", Acc(3)),
            ("jmp -3", Jmp(-3)),
            ("acc -99", Acc(-99)),
            ("acc +1", Acc(1)),
            ("jmp -4", Jmp(-4)),
            ("acc +6", Acc(6)),
        ];

        for (input, instruction) in inputs.into_iter() {
            assert_eq!(input.parse::<Instruction>().unwrap(), instruction);
        }
    }
}
