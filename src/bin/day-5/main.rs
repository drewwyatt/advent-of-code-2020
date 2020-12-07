use std::io::{Error, ErrorKind, Result};

// BFFFBBFRRR: row 70, column 7, seat ID 567.
// FFFBBBFRRR: row 14, column 7, seat ID 119.
// BBFFBBFRLL: row 102, column 4, seat ID 820.

#[derive(Debug)]
struct Tree {
  start: i32,
  size: i32,
}

impl Tree {
  fn done(&self) -> bool {
    self.size == 1
  }

  fn value(&self) -> Result<i32> {
    if self.done() {
      Ok(self.size)
    } else {
      Err(std::io::Error::new(ErrorKind::Other, "value not yet known"))
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

fn main() -> Result<()> {
  let mut tree = Tree {
    start: 0,
    size: 128,
  };
  println!("tree {:?}", tree);
  // BFFFBBFRRR
  // FFFBBBFRRR: row 14, column 7, seat ID 119.
  tree.lower();
  tree.lower();
  tree.lower();
  tree.upper();
  tree.upper();
  tree.upper();
  tree.lower();
  println!("tree {:?}", tree);

  Ok(())
}
