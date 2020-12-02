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

fn part_1(lines: Vec<i32>) -> Option<i32> {
  let mut product: Option<i32> = None;
  for idx in 0..lines.len() {
    let n1 = lines.get(idx).unwrap();
    match product {
      Some(_) => break,
      None => for idx2 in (idx + 1)..lines.len() {
        let n2 = lines.get(idx2).unwrap();
        if n1 + n2 == 2020 {
          product = Some(n1 * n2);
          break;
        }
      }
    }
  }

  product
}

fn part_2(lines: Vec<i32>) -> Option<i32> {
  let mut product: Option<i32> = None;
  for idx in 0..lines.len() {
    if product.is_some() {
      break;
    }

    let n1 = lines.get(idx).unwrap();
    for idx2 in (idx + 1)..lines.len() {
      if product.is_some() {
        break;
      }
      let n2 = lines.get(idx2).unwrap();
      for idx3 in (idx2 + 1)..lines.len() {
        let n3 = lines.get(idx3).unwrap();
        if n1 + n2 + n3 == 2020 {
          product = Some(n1 * n2 * n3);
          break;
        }
      }
    }
  }

  product
}

fn main() -> io::Result<()> {
  let lines = read_input_for_day("day-1")?;
  let args = Cli::from_args();

  let answer = if args.part == 1 { part_1(lines) } else { part_2(lines) };
  println!("Part {} answer: {}", args.part, answer.unwrap());
  Ok(())
}
