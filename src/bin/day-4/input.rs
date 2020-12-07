use regex::{Captures, Regex};
use std::collections::HashSet;
use std::io::{Error, ErrorKind, Result};

const REQUIRED_DOCUMENT_FIELDS: [Field; 7] = [
  Field::BirthYear(None),
  Field::IssueYear(None),
  Field::ExpirationYear(None),
  Field::Height(None),
  Field::HairColor(None),
  Field::EyeColor(None),
  Field::PassportId(None),
];

#[derive(Debug)]
pub struct Document<'a> {
  fields: HashSet<Field<'a>>,
}

impl<'a> Document<'a> {
  fn new() -> Self {
    Document {
      fields: HashSet::new(),
    }
  }

  fn add(&mut self, capture: &Captures<'a>) -> Result<()> {
    let name = self.field_from_str(
      capture.get(1).unwrap().as_str(),
      capture.get(2).unwrap().as_str(),
    );
    match name {
      Some(field) => {
        self.fields.insert(field);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        format!("Error parsing input '{:?}'", capture),
      )),
    }
  }

  fn field_from_str(&self, key: &str, value: &'a str) -> Option<Field<'a>> {
    match key {
      "byr" => Some(Field::BirthYear(Some(value))),
      "iyr" => Some(Field::IssueYear(None)),
      "eyr" => Some(Field::ExpirationYear(None)),
      "hgt" => Some(Field::Height(None)),
      "hcl" => Some(Field::HairColor(None)),
      "ecl" => Some(Field::EyeColor(None)),
      "pid" => Some(Field::PassportId(None)),
      "cid" => Some(Field::CountryId(None)),
      _ => None,
    }
  }

  pub fn is_valid(&self) -> bool {
    REQUIRED_DOCUMENT_FIELDS.iter().fold(
      true,
      |acc, req| {
        if acc {
          self.fields.contains(req)
        } else {
          acc
        }
      },
    )
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Field<'a> {
  BirthYear(Option<&'a str>),
  IssueYear(Option<String>),
  ExpirationYear(Option<String>),
  Height(Option<String>),
  HairColor(Option<String>),
  EyeColor(Option<String>),
  PassportId(Option<String>),
  CountryId(Option<String>), // We don't care about this
}

pub fn documents_from_input<'a>(input: Vec<&'a str>) -> Result<Vec<Document<'a>>> {
  let documents = input.iter().fold(vec![Document::new()], |mut docs, line| {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"([a-z]{3}):(\S+)").unwrap();
    }

    if RE.is_match(line) {
      for cap in RE.captures_iter(line) {
        let doc = docs.last_mut().unwrap();
        doc.add(&cap);
      }
    } else {
      docs.push(Document::new())
    }

    docs
  });
  Ok(documents)
}
