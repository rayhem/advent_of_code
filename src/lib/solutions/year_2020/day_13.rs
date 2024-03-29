use crate::utils::solution::*;
pub struct Day13 {}

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> Option<String> {
        let (departure_estimate, bus_ids) = parse_input(input).ok()?;

        bus_ids
            .iter()
            .filter_map(|bus_id| {
                bus_id.map(|bus| {
                    (
                        bus,
                        (departure_estimate / bus + 1) * bus - departure_estimate,
                    )
                })
            })
            .min_by(|(_, time1), (_, time2)| time1.cmp(time2))
            .map(|(bus, time)| (bus * time).to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

fn parse_input(input: &str) -> Result<(i32, Vec<Option<i32>>), super::AdventError> {
    let mut lines = input.lines();
    Ok((
        lines.next().ok_or(super::AdventError::BadInput)?.parse()?,
        lines
            .next()
            .ok_or(super::AdventError::BadInput)?
            .split(',')
            .map(|bus| bus.parse().ok())
            .collect(),
    ))
}
