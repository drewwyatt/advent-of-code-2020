use aoc::parse_from_captures_or;
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum AdventError {
  InvalidRegex,
  UnrecognizedOperation,
}

#[derive(Debug)]
pub enum Operation {
  Nop,
  Acc,
  Jmp,
}

impl FromStr for Operation {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "nop" => Ok(Self::Nop),
      "acc" => Ok(Self::Acc),
      "jmp" => Ok(Self::Jmp),
      _ => Err(AdventError::UnrecognizedOperation),
    }
  }
}

#[derive(Debug)]
pub struct Instruction {
  pub operation: Operation,
  pub argument: i32,
  pub times_executed: i32,
}

impl FromStr for Instruction {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"(nop|acc|jmp) ([\+\-]\d+)").unwrap();
    }

    let captures = RE.captures(s).ok_or(AdventError::InvalidRegex)?;
    let operation =
      parse_from_captures_or::<Operation, AdventError>(&captures, 1, AdventError::InvalidRegex)?;
    let argument: i32 = parse_from_captures_or(&captures, 2, AdventError::InvalidRegex)?;

    Ok(Instruction {
      operation,
      argument,
      times_executed: 0,
    })
  }
}
