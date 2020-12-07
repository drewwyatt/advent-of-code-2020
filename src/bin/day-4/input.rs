use regex::{Captures, Regex};
use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Document {
  fields: Vec<Field>,
}

impl Document {
  fn push(&mut self, field: Field) {
    self.fields.push(field);
  }
}

#[derive(Debug)]

pub enum FieldName {
  BirthYear,
  IssueYear,
  ExpirationYear,
  Height,
  HairColor,
  EyeColor,
  PassportId,
  CountryId, // We don't care about this
}

#[derive(Debug)]
pub struct Field {
  name: FieldName,
}

impl Field {
  fn from(capture: &Captures) -> Result<Self> {
    let name = Field::name_from_str(capture.get(1).unwrap().as_str());
    match name {
      Some(n) => Ok(Field { name: n }),
      None => Err(Error::new(
        ErrorKind::Other,
        format!("Error parsing input '{:?}'", capture),
      )),
    }
  }

  fn name_from_str(s: &str) -> Option<FieldName> {
    match s {
      "byr" => Some(FieldName::BirthYear),
      "iyr" => Some(FieldName::IssueYear),
      "eyr" => Some(FieldName::ExpirationYear),
      "hgt" => Some(FieldName::Height),
      "hcl" => Some(FieldName::HairColor),
      "ecl" => Some(FieldName::EyeColor),
      "pid" => Some(FieldName::PassportId),
      "cid" => Some(FieldName::CountryId),
      _ => None,
    }
  }
}

pub fn documents_from_input(mut input: Vec<String>) -> Result<Vec<Document>> {
  let documents = input
    .iter()
    .fold(vec![Document { fields: vec![] }], |mut docs, line| {
      lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]{3}):").unwrap();
      }

      if RE.is_match(line) {
        for cap in RE.captures_iter(line) {
          let doc = docs.last_mut().unwrap();
          doc.push(Field::from(&cap).unwrap())
        }
      } else {
        docs.push(Document { fields: vec![] })
      }

      docs
    });
  Ok(documents)
}
