#[macro_use]
extern crate lazy_static;

use aoc::read_input_for_day;
use regex::{Captures, Regex};
use std::io::{Error, ErrorKind, Result};

struct Document {
  fields: Vec<Field>,
}

impl Document {
  fn push(mut self, field: Field) {
    self.fields.push(field);
  }
}

enum Name {
  BirthYear,
  IssueYear,
  ExpirationYear,
  Height,
  HairColor,
  EyeColor,
  PassportId,
  // CountryId, // We don't care about this
}

struct Field {
  name: Name,
}

impl Field {
  fn from(capture: &Captures) -> Result<Self> {
    let name = Field::name_from_str(capture.get(1).unwrap().as_str());
    match name {
      Some(n) => Ok(Field { name: n }),
      None => Err(Error::new(ErrorKind::Other, "Error parsing input")),
    }
  }

  fn name_from_str(s: &str) -> Option<Name> {
    match s {
      "byr" => Some(Name::BirthYear),
      "iyr" => Some(Name::IssueYear),
      "eyr" => Some(Name::ExpirationYear),
      "hgt" => Some(Name::Height),
      "hcl" => Some(Name::HairColor),
      "ecl" => Some(Name::EyeColor),
      "pid" => Some(Name::PassportId),
      _ => None,
    }
  }

  // fn from_match(m: Option<Match<>>) -> Option<Self> {}
}

fn main() -> Result<()> {
  read_input_for_day::<String>("day-4")?.iter().fold(
    vec![Document { fields: vec![] }],
    |mut docs, line| {
      lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]{3}):").unwrap();
      }

      if RE.is_match(line) {
        for cap in RE.captures_iter(line) {
          match docs.last() {
            Some(doc) => doc.push(Field::from(&cap).unwrap()),
            None => (),
          }
        }
      } else {
        docs.push(Document { fields: vec![] })
      }

      docs
    },
  );
  Ok(())
}
