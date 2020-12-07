use std::io::Result;
use std::ops::Range;

// BFFFBBFRRR: row 70, column 7, seat ID 567.
// FFFBBBFRRR: row 14, column 7, seat ID 119.
// BBFFBBFRLL: row 102, column 4, seat ID 820.

// const number_of_rows: i32 = 128;
// const number_of_columns: i32 = 8;

#[derive(Debug)]
struct Tree {
  start: i32,
  size: i32,
}

impl Tree {
  fn done(&self) -> bool {
    self.size == 1
  }

  fn lower(mut self) -> Self {
    self.size = self.size / 2;
    self
  }

  fn upper(mut self) -> Self {
    self.size = self.size / 2;
    let s = self.start;
    self.start = s + (s / 2);
    self
  }
}

fn main() -> Result<()> {
  let tree = Tree {
    start: 0,
    size: 128,
  };
  println!("tree {:?}", tree);
  // BFFFBBFRRR
  tree.upper(); //.upper().upper().upper().lower().lower().upper();
  println!("tree {:?}", tree);

  Ok(())
}
