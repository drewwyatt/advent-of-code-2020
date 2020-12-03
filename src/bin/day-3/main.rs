use aoc;
use std::{
  cmp::PartialEq,
  io::{Error, Result},
  str::FromStr,
};

#[derive(Debug, PartialEq)]
enum Cell {
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
    self.get(index) == Some(&Cell::Tree)
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
  let down = 1;
  let right = 3;
  let mut next_row_index = down;
  let mut cell_index = right;

  let number_of_trees = input.iter().enumerate().fold(0, |mut trees, (index, row)| {
    if index == next_row_index {
      if row.is_tree(cell_index) {
        trees += 1;
      }
      next_row_index += down;
      cell_index += right;
    }

    trees
  });

  println!("The answer is: {}", number_of_trees);

  Ok(())
}
