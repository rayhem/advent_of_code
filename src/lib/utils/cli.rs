use clap::{App, Arg, ArgMatches};
use std::{num::ParseIntError, str::FromStr};

const YEARS: &str = "years";
const DAYS: &str = "days";
const INPUT: &str = "input";

#[derive(Clone, Debug, Default)]
pub struct CommandLineInterface {
    iface: ArgMatches<'static>,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        Self {
            iface: App::new("Advent of Code")
                .version("v0.1")
                .arg(
                    Arg::with_name(YEARS)
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
                    Arg::with_name(DAYS)
                        .default_value("1-25")
                        .help("Specify daily solutions to execute")
                        .long("day")
                        .short("d")
                        .takes_value(true)
                        .use_delimiter(true)
                        .validator(validate_int_sequence)
                        .value_name("[1-25]"),
                )
                .arg(
                    Arg::with_name(INPUT)
                        .default_value(".")
                        .help(
                            "Path to input files directory (assumes subdirectories named by year)",
                        )
                        .long(INPUT)
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
                .get_matches(),
        }
    }

    pub fn get_years(&self) -> Vec<i32> {
        self.get_int_sequence(YEARS)
    }

    pub fn get_days(&self) -> Vec<i32> {
        self.get_int_sequence(DAYS)
    }

    pub fn get_input_dir(&self) -> std::path::PathBuf {
        self.iface.value_of(INPUT).unwrap().into()
    }

    fn get_int_sequence(&self, flag: &str) -> Vec<i32> {
        match self.iface.values_of(flag) {
            Some(v) => v
                .flat_map(IntSpecifier::from_str)
                .fold(Vec::new(), |mut acc, d| {
                    match d {
                        IntSpecifier::Single(n) => acc.push(n),
                        IntSpecifier::Range(l, u) => acc.extend(l..=u),
                    };
                    acc
                }),
            None => Vec::new(),
        }
    }
}

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
