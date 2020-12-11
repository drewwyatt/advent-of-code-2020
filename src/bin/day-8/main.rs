#[macro_use]
extern crate lazy_static;
mod input;

use aoc::{read_input_for_day, read_part_for_day};
use input::{Instruction, Operation};
use std::convert::TryFrom;
use std::io;

fn part_one(mut input: Vec<Instruction>) -> i32 {
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

  acc
}

fn part_2(mut input: Vec<Instruction>) -> i32 {
  let mut acc = 0;
  for mut inst in input {
    if inst.operation != Operation::Acc {
      let orig_operation = inst.operation;
      inst.operation = if inst.operation == Operation::Jmp {
        Operation::Nop
      } else {
        Operation::Jmp
      };

      inst.operation = orig_operation;
      // TODO: reset executions?
    }
  }

  acc
}

fn main() -> io::Result<()> {
  let input = read_input_for_day::<Instruction>("day-8")?;
  let part = read_part_for_day();
  let answer = if part == 1 {
    part_one(input)
  } else {
    part_2(input)
  };

  println!("Day-8 part {} answer: {}", part, answer);
  Ok(())
}
