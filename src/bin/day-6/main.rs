use aoc::read_input_for_day;
use std::{collections::HashSet, io};

struct Questionnaire {
  data: HashSet<char>,
}

impl Questionnaire {
  fn new() -> Self {
    Questionnaire {
      data: HashSet::new(),
    }
  }

  fn check(&mut self, answers: &str) {
    for c in answers.chars() {
      self.data.insert(c);
    }
  }

  fn len(&self) -> usize {
    self.data.len()
  }
}

fn main() -> io::Result<()> {
  let number_of_affirmatives = read_input_for_day::<String>("day-6")?
    .iter()
    .fold(vec![Questionnaire::new()], |mut questionnaires, line| {
      if line.len() > 0 {
        let q = questionnaires.last_mut().unwrap();
        q.check(line);
      } else {
        questionnaires.push(Questionnaire::new());
      }

      questionnaires
    })
    .iter()
    .fold(0, |acc, q| acc + q.len());

  println!("answer: {}", number_of_affirmatives);
  Ok(())
}
