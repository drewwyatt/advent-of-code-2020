use aoc::read_input_for_day;
use std::io::Result;

fn main() -> Result<()> {
  let lines = read_input_for_day("day-1")?;
  let mut product: Option<i32> = None;

  for idx in 0..lines.len() {
    let n1 = lines.get(idx).unwrap();
    match product {
      Some(_) => break,
      None => for idx2 in idx..lines.len() {
        let n2 = lines.get(idx2).unwrap();
        if n1 + n2 == 2020 {
          product = Some(n1 * n2);
          break;
        }
      }
    }
  }

  println!("Answer: {}", product.unwrap());
  Ok(())
}
