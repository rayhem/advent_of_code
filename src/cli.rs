use clap::{App, Arg, ArgMatches};
use std::collections::HashSet;
use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, Debug)]
enum DaySpecifier {
    Single(i32),
    Range(i32, i32),
}

impl FromStr for DaySpecifier {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once('-') {
            let v1 = a.parse()?;
            let v2 = b.parse()?;
            Ok(DaySpecifier::Range(
                std::cmp::min(v1, v2),
                std::cmp::max(v1, v2),
            ))
        } else {
            Ok(DaySpecifier::Single(s.parse()?))
        }
    }
}

fn validate_day_specifier(s: String) -> Result<(), String> {
    DaySpecifier::from_str(&s)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

pub fn make_cli() -> ArgMatches<'static> {
    App::new("Advent of Code 2015")
        .version("0.1")
        .arg(
            Arg::with_name("days")
                .default_value("1-25")
                .help("Specify solutions to execute")
                .long("days")
                .short("d")
                .takes_value(true)
                .use_delimiter(true)
                .validator(validate_day_specifier)
                .value_name("[1-25]"),
        )
        .get_matches()
}

fn get_day_specifiers_from_cli(matches: &ArgMatches) -> Vec<DaySpecifier> {
    matches
        .values_of("days")
        .map(|v| {
            v.map(|s| DaySpecifier::from_str(s).unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap()
}

pub fn get_cli_days(matches: &ArgMatches) -> Vec<i32> {
    let day_specifiers = get_day_specifiers_from_cli(matches);

    let mut daySet: HashSet<i32> = HashSet::new();
    for specifier in day_specifiers.into_iter() {
        match specifier {
            DaySpecifier::Single(n) => {
                daySet.insert(n);
            }
            DaySpecifier::Range(lbound, ubound) => {
                for i in lbound..=ubound {
                    daySet.insert(i);
                }
            }
        }
    }

    let mut v = daySet.into_iter().collect::<Vec<_>>();
    v.sort();

    v
}
