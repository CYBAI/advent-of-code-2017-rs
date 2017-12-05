extern crate aoc17;

use aoc17::day02::{checksum, evenly_divisible_values};
use std::fs::File;
use std::io::prelude::*;

fn main() {
  run_part_1();
  run_part_2();
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

fn run_part_2() {
  let input = "5 9 2 8
9 4 7 3
3 8 6 5";

  assert_eq!(9, evenly_divisible_values(input));

  let mut f = File::open("./src/day02/input-part-2").expect("FILE NOT FOUND");

  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

  println!("{}", evenly_divisible_values(contents.as_str()));
}