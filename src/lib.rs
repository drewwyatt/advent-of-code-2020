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
