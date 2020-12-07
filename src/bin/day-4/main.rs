#[macro_use]
extern crate lazy_static;
mod input;

use aoc::{read_input_for_day, read_part_for_day};
use input::documents_from_input;
use std::io::Result;

fn main() -> Result<()> {
  let part = read_part_for_day();
  let input = read_input_for_day::<String>("day-4")?;
  let documents = documents_from_input(input.iter().map(|s| s.as_ref()).collect::<Vec<&str>>())?;
  let valid_documents = documents.iter().filter(|d| d.is_valid()).count();
  println!("Day-4 answer: {}", valid_documents);
  Ok(())
}
