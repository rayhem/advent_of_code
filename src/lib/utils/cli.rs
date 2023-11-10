use clap::{App, Arg, ArgMatches};
use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, Debug)]
enum IntSpecifier {
    Single(i32),
    Range(i32, i32),
}

impl FromStr for IntSpecifier {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once('-') {
            let v1 = a.parse()?;
            let v2 = b.parse()?;
            Ok(IntSpecifier::Range(
                std::cmp::min(v1, v2),
                std::cmp::max(v1, v2),
            ))
        } else {
            Ok(IntSpecifier::Single(s.parse()?))
        }
    }
}

fn validate_int_sequence(s: String) -> Result<(), String> {
    IntSpecifier::from_str(&s)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

pub fn make_cli() -> ArgMatches<'static> {
    App::new("Advent of Code 2015")
        .version("0.1")
        .arg(
            Arg::with_name("years")
                .default_value("2015-2022")
                .help("Specify yearly solutions to execute")
                .long("year")
                .short("y")
                .takes_value(true)
                .use_delimiter(true)
                .validator(validate_int_sequence)
                .value_name("[2015-2022]"),
        )
        .arg(
            Arg::with_name("days")
                .default_value("1-25")
                .help("Specify daily solutions to execute")
                .long("days")
                .short("d")
                .takes_value(true)
                .use_delimiter(true)
                .validator(validate_int_sequence)
                .value_name("[1-25]"),
        )
        .arg(
            Arg::with_name("input")
                .default_value("./input")
                .help("Path to input files directory (assumes subdirectories named by year)")
                .long("input")
                .short("i")
                .takes_value(true)
                .validator(|dir| {
                    if std::path::Path::is_dir(std::path::Path::new(&dir)) {
                        Ok(())
                    } else {
                        Err("Input directory does not exist".to_string())
                    }
                })
                .value_name("PATH"),
        )
        .get_matches()
}

pub fn get_cli_int_sequence(flag: &str, matches: &ArgMatches) -> Vec<i32> {
    if let Some(v) = matches.value_of(flag) {
        v.split(',')
            .flat_map(IntSpecifier::from_str)
            .fold(Vec::new(), |mut acc, d| {
                match d {
                    IntSpecifier::Single(n) => acc.push(n),
                    IntSpecifier::Range(l, u) => acc.extend(l..=u),
                };
                acc
            })
    } else {
        Vec::new()
    }
}
