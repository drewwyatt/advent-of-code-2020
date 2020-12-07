#[macro_use]
extern crate lazy_static;
mod input;

use aoc::read_input_for_day;
use input::documents_from_input;
use std::io::Result;

fn main() -> Result<()> {
  let documents = documents_from_input(read_input_for_day::<String>("day-4")?)?;
  println!("docs: {:?}", documents);
  Ok(())
}
