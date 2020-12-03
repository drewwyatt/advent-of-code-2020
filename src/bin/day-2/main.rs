#[macro_use]
extern crate lazy_static;
mod input;

use aoc;
use input::Part1Line;
use std::io;

fn main() -> io::Result<()> {
    let input = aoc::read_input_for_day::<Part1Line>("day-2")?;

    let count = input
        .iter()
        .filter(|i| {
            i.count_range
                .contains(&(i.password.chars().filter(|c| c == &(i.character)).count() as i32))
        })
        .count();

    println!("Answer: {}", count);
    Ok(())
}
