mod input;

use aoc::read_input_for_day;
use input::BoardingPass;
use std::io::Result;

// BFFFBBFRRR: row 70, column 7, seat ID 567.
// FFFBBBFRRR: row 14, column 7, seat ID 119.
// BBFFBBFRLL: row 102, column 4, seat ID 820.

fn main() -> Result<()> {
  // let boarding_passes = read_input_for_day::<BoardingPass>("day-5")?;
  let bp = BoardingPass::new("BFFFBBFRRR").unwrap();
  // println!("boarding pass: {:?}", boarding_passes.get(0));
  println!("{:?}", bp);
  println!(
    "row: {:?}, seat: {:?}, id: {:?}",
    bp.row(),
    bp.seat(),
    bp.id()
  );
  Ok(())
}
