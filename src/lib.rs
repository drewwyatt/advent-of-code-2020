use std::fs;

pub fn read_input_for_day(day: &str) -> std::io::Result<Vec<i32>> {
  let path = format!("./src/bin/{}/INPUT", day);
  let contents = fs::read_to_string(path)?;

  let values = contents
    .lines()
    .map(|l| l.parse().unwrap())
    .collect::<Vec<i32>>();

  Ok(values)
}

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

pub fn read_part_for_day() -> i32 {
  Cli::from_args().part
}
