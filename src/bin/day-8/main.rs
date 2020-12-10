#[macro_use]
extern crate lazy_static;
mod input;

use aoc::read_input_for_day;
use input::{Instruction, Operation};
use std::convert::TryFrom;
use std::io;

fn main() -> io::Result<()> {
  let mut input = read_input_for_day::<Instruction>("day-8")?;
  let mut index = 0;
  let mut acc = 0;

  loop {
    let instruction = input.get_mut(index).unwrap();
    if instruction.times_executed > 0 {
      break;
    }

    match instruction.operation {
      Operation::Acc => {
        index += 1;
        acc += instruction.argument;
      }
      Operation::Jmp => {
        let mut i32_index = i32::try_from(index).unwrap();
        i32_index += instruction.argument;
        index = usize::try_from(i32_index).unwrap();
      }
      Operation::Nop => {
        index += 1;
      }
    }

    instruction.times_executed += 1;
  }

  println!("Answer: {}", acc);
  Ok(())
}
