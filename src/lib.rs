use std::{
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
};

pub fn lines_from_input(day_and_file: &str) -> io::Result<Vec<String>> {
  BufReader::new(File::open(Path::new("./src/bin/").join(day_and_file))?).lines().collect()
}
