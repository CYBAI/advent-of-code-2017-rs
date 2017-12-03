extern crate aoc17;

use aoc17::day02::checksum;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  run_part_1();
}

fn run_part_1() {
  let input = "5 1 9 5
7 5 3
2 4 6 8";

  assert_eq!(18, checksum(input));

  let mut f = File::open("./src/day02/input").expect("FILE NOT FOUND");

  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

  println!("{}", checksum(contents.as_str()));
}