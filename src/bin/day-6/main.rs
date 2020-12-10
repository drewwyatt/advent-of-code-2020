mod input;

use aoc::{read_input_for_day, read_part_for_day};
use input::{Part1, Part2, Questionnaire};
use std::io;

fn calculate<T>(input: Vec<String>) -> usize
where
  T: Questionnaire,
{
  input
    .iter()
    .fold(vec![T::new()], |mut questionnaires, line| {
      if line.len() > 0 {
        let q = questionnaires.last_mut().unwrap();
        q.add(line);
      } else {
        questionnaires.push(T::new());
      }

      questionnaires
    })
    .iter()
    .fold(0, |acc, q| acc + q.calculate())
}

fn main() -> io::Result<()> {
  let input = read_input_for_day::<String>("day-6")?;
  let part = read_part_for_day();

  let answer = if part == 1 {
    calculate::<Part1>(input)
  } else {
    calculate::<Part2>(input)
  };

  println!("Day 6 Part {} answer: {}", part, answer);
  Ok(())
}
