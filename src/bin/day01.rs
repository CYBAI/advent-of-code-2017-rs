extern crate aoc17;

use aoc17::day01::parse_sequence;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  assert_eq!(0, parse_sequence("1234"));
  assert_eq!(3, parse_sequence("1122"));
  assert_eq!(4, parse_sequence("1111"));
  assert_eq!(9, parse_sequence("9123456789"));

  let mut f = File::open("./src/day01/input").expect("FILE NOT FOUND");

  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

  println!("{}", parse_sequence(contents.as_str()));
}