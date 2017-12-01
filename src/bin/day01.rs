extern crate aoc17;

use std::io::stdin;
use aoc17::day01::parse_sequence;

fn main() {
  println!("Please input a number to parse:");

  let mut input = String::new();
  match stdin().read_line(&mut input) {
    Ok(_n) => {
      println!("{}", parse_sequence(input.as_str()));
    },
    Err(error) => println!("error: {}", error),
  }
}