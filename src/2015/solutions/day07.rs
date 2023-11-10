use std::{collections::HashMap, str::FromStr};
use utils::solution::Solution;

pub struct Day07 {}

type Number = u16;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            Wires::from_str(input)
                .unwrap()
                .eval(Operand::Name("a".to_string()))
                .to_string(),
        )
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

#[derive(Clone, Debug)]
struct Wires(HashMap<String, Operation>);

impl Wires {
    fn eval(&mut self, op: Operand) -> Number {
        match op {
            Operand::Value(value) => value,
            Operand::Name(s) => {
                let result = match self.0.get(&s).cloned().unwrap() {
                    Operation::And(a, b) => self.eval(a) & self.eval(b),
                    Operation::Assign(a) => self.eval(a),
                    Operation::Not(a) => !self.eval(a),
                    Operation::Or(a, b) => self.eval(a) | self.eval(b),
                    Operation::ShiftL(a, b) => self.eval(a) << self.eval(b),
                    Operation::ShiftR(a, b) => self.eval(a) >> self.eval(b),
                };

                self.0.insert(s, Operation::Assign(result.into()));
                result
            }
        }
    }
}

impl FromStr for Wires {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wires = HashMap::new();

        for line in s.lines() {
            let (op, destination) = line.split_once(" -> ").unwrap();
            wires.insert(destination.to_string(), Operation::from_str(op).unwrap());
        }

        Ok(Wires(wires))
    }
}

#[derive(Clone, Debug)]
enum Operand {
    Value(Number),
    Name(String),
}

impl From<Number> for Operand {
    fn from(value: Number) -> Self {
        Self::Value(value)
    }
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<Number>()
            .map(From::from)
            .unwrap_or(Self::Name(s.to_string())))
    }
}

#[derive(Clone, Debug)]
enum Operation {
    And(Operand, Operand),
    Assign(Operand),
    Not(Operand),
    Or(Operand, Operand),
    ShiftL(Operand, Operand),
    ShiftR(Operand, Operand),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect::<Vec<_>>();

        Ok(match tokens.len() {
            1 => Operation::Assign(Operand::from_str(tokens[0]).unwrap()),
            2 => Operation::Not(Operand::from_str(tokens[1]).unwrap()),
            3 => {
                let lhs = Operand::from_str(tokens[0]).unwrap();
                let rhs = Operand::from_str(tokens[2]).unwrap();

                match tokens[1] {
                    "AND" => Operation::And(lhs, rhs),
                    "OR" => Operation::Or(lhs, rhs),
                    "LSHIFT" => Operation::ShiftL(lhs, rhs),
                    "RSHIFT" => Operation::ShiftR(lhs, rhs),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        const INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        let mut wires = Wires::from_str(INPUT).unwrap();

        assert_eq!(wires.eval(Operand::Name("d".to_string())), 72);
        assert_eq!(wires.eval(Operand::Name("e".to_string())), 507);
        assert_eq!(wires.eval(Operand::Name("f".to_string())), 492);
        assert_eq!(wires.eval(Operand::Name("g".to_string())), 114);
        assert_eq!(wires.eval(Operand::Name("h".to_string())), 65412);
        assert_eq!(wires.eval(Operand::Name("i".to_string())), 65079);
        assert_eq!(wires.eval(Operand::Name("x".to_string())), 123);
        assert_eq!(wires.eval(Operand::Name("y".to_string())), 456);
    }
}
