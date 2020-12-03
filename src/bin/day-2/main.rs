#[macro_use]
extern crate lazy_static;
use aoc;
use regex::{Captures, Regex};
use std::io;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
enum AdventError {
    InvalidRegex,
}

#[derive(Debug)]
struct Day2Line {
    count_range: Range<i32>,
    character: char,
    password: String,
}

impl FromStr for Day2Line {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)\-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        }
        let captures = RE.captures(s).ok_or(AdventError::InvalidRegex)?;

        let count_start = parse_from_captures(&captures, 1)?;
        let count_end = parse_from_captures(&captures, 2)?;
        Ok(Day2Line {
            count_range: count_start..count_end,
            character: parse_from_captures(&captures, 3)?,
            password: parse_from_captures(&captures, 4)?,
        })
    }
}

fn parse_from_captures<T>(captures: &Captures, index: usize) -> Result<T, AdventError>
where
    T: FromStr,
{
    captures
        .get(index)
        .ok_or(AdventError::InvalidRegex)?
        .as_str()
        .parse::<T>()
        .map_err(|_| AdventError::InvalidRegex)
}

fn main() -> io::Result<()> {
    let input = aoc::read_input_for_day::<Day2Line>("day-2")?;
    println!("{:?}", input.get(0));
    Ok(())
}
