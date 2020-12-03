use aoc;
use std::{
  io::{Error, Result},
  str::FromStr,
};

pub enum Cell {
  Tree,
  Snow,
}

struct Row {
  cells: Vec<Cell>,
}

impl Row {
  fn get(&self, index: usize) -> Option<&Cell> {
    self.cells.get(index % self.cells.len())
  }

  fn is_tree(&self, index: usize) -> bool {
    self.get(index) == &(Some(Cell::Tree))
  }
}

impl FromStr for Row {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    Ok(Row {
      cells: s
        .chars()
        .map(|c| if c == '.' { Cell::Snow } else { Cell::Tree })
        .collect(),
    })
  }
}

fn main() -> Result<()> {
  let input = aoc::read_input_for_day::<Row>("day-3")?;
  let right = 3;
  let mut index = 0;
  let mut trees = 0;

  for row in input.iter() {
    index += right;
    if row.is_tree(index) {
      trees += 1;
    }
  }

  Ok(())
}
