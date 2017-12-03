//!
//! Implementation of Day 2 in Advent of Code 2017
//!

pub fn checksum(input: &str) -> i32 {
  input.lines()
    .fold(0, |acc, line| {
      acc + (find_max(line) - find_min(line))
    })
}

fn find_max(line: &str) -> i32 {
  line.split_whitespace()
      .map(|n| n.parse::<i32>().unwrap())
      .max()
      .unwrap()
}

fn find_min(line: &str) -> i32 {
  line.split_whitespace()
      .map(|n| n.parse::<i32>().unwrap())
      .min()
      .unwrap()
}