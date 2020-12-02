use aoc::read_input_for_day;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
  about = "Solutions for Advent of Code 2020",
  author = "Drew Wyatt <drew.j.wyatt@gmail.com>",
  name = "advent-of-code-2020"
)]
struct Cli {
  #[structopt(
    default_value = "1",
    help = "Part 1 or 2",
    short,
    long
  )]
  part: i32,
}


fn main() -> io::Result<()> {
  let lines = read_input_for_day("day-1")?;
  let mut product: Option<i32> = None;

  let args = Cli::from_args();
  println!("part: {}", args.part);

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
