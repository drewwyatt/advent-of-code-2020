use std::io;
use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {}

#[derive(Debug)]

pub struct BoardingPass {
  row_code: String,
  seat_code: String,
}

impl FromStr for BoardingPass {
  type Err = AdventError;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let mut row_code = String::from("");
    let mut seat_code = String::from("");
    for (index, char) in line.chars().enumerate() {
      if index < 7 {
        row_code.push(char);
      } else {
        seat_code.push(char);
      }
    }

    Ok(BoardingPass {
      row_code,
      seat_code,
    })
  }
}

// Tree Stuff

#[derive(Debug)]
struct Tree {
  start: i32,
  size: i32,
}

impl Tree {
  fn done(&self) -> bool {
    self.size == 1
  }

  fn value(&self) -> io::Result<i32> {
    if self.done() {
      Ok(self.size)
    } else {
      Err(io::Error::new(io::ErrorKind::Other, "value not yet known"))
    }
  }

  fn lower(&mut self) -> &Self {
    self.size = self.size / 2;
    self
  }

  fn upper(&mut self) -> &Self {
    self.size = self.size / 2;
    self.start += self.size;
    self
  }
}

// let mut tree = Tree {
//   start: 0,
//   size: 128,
// };
// println!("tree {:?}", tree);
// // BFFFBBFRRR
// // FFFBBBFRRR: row 14, column 7, seat ID 119.
// tree.lower();
// tree.lower();
// tree.lower();
// tree.upper();
// tree.upper();
// tree.upper();
// tree.lower();
// println!("tree {:?}", tree);
