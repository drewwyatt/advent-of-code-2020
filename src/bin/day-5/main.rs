mod input;

use aoc::read_input_for_day;
use input::BoardingPass;

fn main() -> std::io::Result<()> {
  let boarding_passes = read_input_for_day::<BoardingPass>("day-5")?;
  let mut highest_id = 0;
  for pass in boarding_passes.iter() {
    let id = pass.id().unwrap();
    if id > highest_id {
      highest_id = id;
    }
  }

  println!("the highest id is: {}", highest_id);
  Ok(())
}
