use std::collections::HashSet;

pub trait Questionnaire {
  fn new() -> Self;
  fn add(&mut self, answers: &str);
  fn calculate(&self) -> usize;
}

pub struct Part1 {
  data: HashSet<char>,
}

impl Questionnaire for Part1 {
  fn new() -> Self {
    Part1 {
      data: HashSet::new(),
    }
  }

  fn add(&mut self, answers: &str) {
    for c in answers.chars() {
      self.data.insert(c);
    }
  }

  fn calculate(&self) -> usize {
    self.data.len()
  }
}

pub struct Part2 {
  data: Vec<HashSet<char>>,
}

impl Questionnaire for Part2 {
  fn new() -> Self {
    Part2 { data: vec![] }
  }

  fn add(&mut self, answers: &str) {
    let mut person = HashSet::new();
    for c in answers.chars() {
      person.insert(c);
    }
    self.data.push(person);
  }

  fn calculate(&self) -> usize {
    let first = self.data.first().unwrap();
    let mut total = 0;
    for answer in first {
      if self.data.iter().all(|d| d.contains(answer)) {
        total += 1;
      }
    }

    total
  }
}
