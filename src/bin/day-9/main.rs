use aoc::{read_input_for_day, read_part_for_day};
use std::io;

fn part_one(input: &Vec<i64>, preamble_size: usize) -> Result<i64, ()> {
  let mut result = Err(());
  for idx in preamble_size..input.len() {
    let expected = input.get(idx).unwrap();
    let start = idx - preamble_size;
    let prev_range = &input[start..idx];
    let mut found_pair = false;
    for n in prev_range {
      let difference = expected - n;
      if prev_range.contains(&difference) {
        found_pair = true;
        break;
      }
    }

    if !found_pair {
      result = Ok(*expected);
    }
  }

  result
}

fn part_two(input: &Vec<i64>, expected_sum: i64) -> Result<i64, ()> {
  let mut result = Err(());
  let mut contig_set: Vec<&i64>;
  for start in 0..input.len() {
    let mut sum = 0;
    contig_set = vec![];
    for idx in start..input.len() {
      let n = input.get(idx).unwrap();
      sum += n;
      contig_set.push(n);

      if sum == expected_sum {
        contig_set.sort();
        let lowest = contig_set.first().unwrap();
        let highest = contig_set.last().unwrap();
        result = Ok(*lowest + *highest);
        break;
      } else if sum > expected_sum {
        break;
      }
    }

    if result.is_ok() {
      break;
    }
  }

  result
}

fn main() -> io::Result<()> {
  let input = read_input_for_day::<i64>("day-9")?;
  let part = read_part_for_day();
  let answer = if part == 1 {
    part_one(&input, 25).unwrap()
  } else {
    part_two(&input, 14144619).unwrap()
  };

  println!("Day-9 part {} answer: {}", part, answer);
  Ok(())
}
