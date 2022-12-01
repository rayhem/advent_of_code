use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day08 {}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            VirtualMachine::from(parse_input(input))
                .run()
                .acc
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let tape = parse_input(input);
        Some(
            tape.iter()
                .enumerate()
                .filter(|(_, op)| !matches!(op, Instruction::Acc(_)))
                .map(|(i, _)| VirtualMachine::from(swap_instruction(tape.clone(), i)).run())
                .filter(|vm| vm.is_ok())
                .map(|vm| vm.acc)
                .next()
                .unwrap()
                .to_string(),
        )
    }
}

fn parse_input(input: &str) -> Tape {
    input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Tape>()
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
    Nop(i32),
}

impl std::str::FromStr for Instruction {
    type Err = Day08Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, arg) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or(Day08Error::BadCommand)?;

        let val = arg.parse().unwrap();

        match cmd {
            "acc" => Ok(Instruction::Acc(val)),
            "jmp" => Ok(Instruction::Jmp(val)),
            "nop" => Ok(Instruction::Nop(val)),
            _ => Err(Day08Error::UnknownCommand),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    NotStarted,
    Executing,
    InfiniteLoop,
    Failed,
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

impl From<Tape> for VirtualMachine {
    fn from(tape: Tape) -> Self {
        let n = tape.len();
        VirtualMachine {
            instructions: tape,
            hit_count: vec![0; n],
            status: Status::NotStarted,
            ip: 0,
            acc: 0,
        }
    }
}

impl VirtualMachine {
    fn run(mut self) -> Self {
        while self.status != Status::Terminated && self.status != Status::InfiniteLoop {
            self.step();
        }

        self
    }

    fn step(&mut self) {
        self.set_status();

        if self.status == Status::Executing {
            self.hit_count[self.ip as usize] += 1;
            self.run_current_instruction();
        }
    }

    fn set_status(&mut self) {
        let ip = self.ip as usize;

        self.status = match ip {
            _ if ip == self.instructions.len() => Status::Terminated,
            _ if ip > self.instructions.len() => Status::Failed,
            _ => match self.hit_count[ip] {
                i if i == 0 => Status::Executing,
                _ => Status::InfiniteLoop,
            },
        }
    }

    fn run_current_instruction(&mut self) {
        match self.instructions[self.ip as usize] {
            Instruction::Acc(n) => {
                self.acc += n;
                self.ip += 1
            }
            Instruction::Jmp(n) => self.ip += n,
            Instruction::Nop(_) => self.ip += 1,
        };
    }

    fn is_ok(&self) -> bool {
        match self.status {
            Status::InfiniteLoop => false,
            Status::Failed => false,
            _ => true,
        }
    }
}

fn swap_instruction(mut tape: Tape, i: usize) -> Tape {
    use Instruction::*;
    tape[i] = match tape[i] {
        Jmp(n) => Nop(n),
        Nop(n) => Jmp(n),
        Acc(n) => Acc(n),
    };

    tape
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parse() {
        use Instruction::*;
        let inputs = vec![
            ("nop +0", Nop(0)),
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
