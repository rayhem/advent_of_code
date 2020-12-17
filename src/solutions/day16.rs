use super::AdventError;
use crate::solutions::Solution;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub struct Day16 {}

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> Option<String> {
        let (constraints, _, tickets) = parse_input(input).unwrap();

        Some(
            tickets
                .iter()
                .map(|ticket| match ticket.status(&constraints) {
                    TicketState::Invalid(n) => n,
                    TicketState::Valid => 0,
                })
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let (constraints, my_ticket, tickets) = parse_input(input).unwrap();

        let tickets: Vec<Ticket> = tickets
            .into_iter()
            .filter(|ticket| ticket.status(&constraints).into())
            .collect();

        assert!(tickets
            .iter()
            .all(|ticket| ticket.values.len() == tickets[0].values.len()));

        let fields = associated_fields(&mut potential_fields(&tickets, &constraints));

        Some(
            fields
                .iter()
                .filter(|(name, _)| name.starts_with("departure"))
                .map(|(_, &i)| my_ticket.values[i] as usize)
                .product::<usize>()
                .to_string(),
        )
    }
}

fn potential_fields(
    tickets: &[Ticket],
    constraints: &[Constraint],
) -> Vec<(String, HashSet<usize>)> {
    let mut valid_fields = HashMap::new();
    for field_idx in 0..tickets[0].values.len() {
        for constraint in constraints {
            if tickets
                .iter()
                .all(|ticket| constraint.validate(ticket.values[field_idx]))
            {
                let entry = valid_fields
                    .entry(constraint.name.to_string())
                    .or_insert_with(HashSet::new);
                entry.insert(field_idx);
            }
        }
    }

    valid_fields.into_iter().collect()
}

fn associated_fields(fields: &mut Vec<(String, HashSet<usize>)>) -> HashMap<String, usize> {
    fields.sort_by(|(_, first), (_, second)| second.len().cmp(&first.len()));

    let n = fields.len();

    let mut deduced_fields = HashMap::new();
    for i in 0..(n - 1) {
        deduced_fields.insert(
            fields[i].0.clone(),
            *fields[i].1.difference(&fields[i + 1].1).next().unwrap(),
        );
    }

    deduced_fields.insert(
        fields[n - 1].0.clone(),
        fields[n - 1].1.drain().next().unwrap(),
    );

    deduced_fields
}

fn parse_input(input: &str) -> Result<(Vec<Constraint>, Ticket, Vec<Ticket>), AdventError> {
    let mut parts = input.split("\n\n");

    let constraints = parts
        .next()
        .ok_or(AdventError::BadInput)?
        .trim()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let my_ticket = parts
        .next()
        .ok_or(AdventError::BadInput)?
        .trim()
        .lines()
        .last()
        .unwrap()
        .parse()?;

    let tickets = parts
        .next()
        .ok_or(AdventError::BadInput)?
        .trim()
        .lines()
        .skip(1) // Skip "nearby tickets: "
        .filter_map(|line| line.parse().ok())
        .collect();

    Ok((constraints, my_ticket, tickets))
}

#[derive(Clone, Debug)]
struct Ticket {
    values: Vec<i32>,
}

#[derive(Clone, Copy, Debug)]
enum TicketState {
    Valid,
    Invalid(i32),
}

impl From<TicketState> for bool {
    fn from(ticket_state: TicketState) -> Self {
        match ticket_state {
            TicketState::Valid => true,
            TicketState::Invalid(_) => false,
        }
    }
}

impl Ticket {
    fn status(&self, constraints: &[Constraint]) -> TicketState {
        match self
            .values
            .iter()
            .filter(|&&value| {
                !constraints
                    .iter()
                    .any(|constraint| constraint.validate(value))
            })
            .sum()
        {
            0 => TicketState::Valid,
            error_rate => TicketState::Invalid(error_rate),
        }
    }
}

impl FromStr for Ticket {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            values: s.split(',').filter_map(|v| v.parse().ok()).collect(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    low: i32,
    high: i32,
}

impl Range {
    fn contains(&self, value: i32) -> bool {
        (self.low <= value) && (value <= self.high)
    }
}

impl FromStr for Range {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        Ok(Range {
            low: parts.next().ok_or(AdventError::BadInput)?.parse()?,
            high: parts.next().ok_or(AdventError::BadInput)?.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
struct Constraint {
    name: String,
    ranges: (Range, Range),
}

impl Constraint {
    fn validate(&self, value: i32) -> bool {
        self.ranges.0.contains(value) || self.ranges.1.contains(value)
    }
}

impl FromStr for Constraint {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let name = parts.next().ok_or(AdventError::BadInput)?.to_string();
        let mut constraint_specs = parts.next().ok_or(AdventError::BadInput)?.split(" or ");

        Ok(Constraint {
            name,
            ranges: (
                constraint_specs
                    .next()
                    .ok_or(AdventError::BadInput)?
                    .parse()?,
                constraint_specs
                    .next()
                    .ok_or(AdventError::BadInput)?
                    .parse()?,
            ),
        })
    }
}
