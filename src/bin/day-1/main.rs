use aoc::lines_from_input;

fn main() {
  let lines = lines_from_input("day-1/INPUT");
  match lines {
    Err(e) => println!("oh no {:?}", e),
    Ok(v) => println!("{:?}", v.get(0))
  }
}