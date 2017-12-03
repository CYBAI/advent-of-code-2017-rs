extern crate aoc17;

use aoc17::day01::{parse_sequence, halfway_around};
use std::fs::File;
use std::io::prelude::*;

fn main() {
  run_part_1();
  run_part_2();
}

fn run_part_1() {
  assert_eq!(0, parse_sequence("1234"));
  assert_eq!(3, parse_sequence("1122"));
  assert_eq!(4, parse_sequence("1111"));
  assert_eq!(9, parse_sequence("9123456789"));

  let mut f = File::open("./src/day01/input").expect("FILE NOT FOUND");

  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

  println!("{}", parse_sequence(contents.as_str()));
}

fn run_part_2() {
  assert_eq!(6, halfway_around("1212"));
  assert_eq!(0, halfway_around("1122"));
  assert_eq!(4, halfway_around("123425"));
  assert_eq!(12, halfway_around("123123"));
  assert_eq!(4, halfway_around("12131415"));

  let mut f = File::open("./src/day01/input-part-2").expect("FILE NOT FOUND");

  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

  println!("{}", halfway_around(contents.as_str()));
}