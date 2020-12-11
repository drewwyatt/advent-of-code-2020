use aoc::read_input_for_day;
use std::io;

fn part_one<'a>(input: &'a Vec<i64>, preamble_size: usize) -> Result<&'a i64, ()> {
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
      result = Ok(expected);
    }
  }

  result
}

fn main() -> io::Result<()> {
  let input = read_input_for_day::<i64>("day-9")?;
  let answer = part_one(&input, 25).unwrap();

  println!("Day-9 part 1 answer: {}", answer);
  Ok(())
}
