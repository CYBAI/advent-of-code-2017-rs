//!
//! Implementation of Day 2 in Advent of Code 2017
//!

use std::cmp;
use std::collections::{HashMap, HashSet};

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

pub fn evenly_divisible_values(input: &str) -> i32 {
  let vectors: Vec<Vec<i32>> = parse_input(input);

  vectors.into_iter()
    .fold(0, |acc, v| {
      acc + find_evenly_divisible(v)
    })
}

fn find_evenly_divisible(vec: Vec<i32>) -> i32 {
  let mut arr: Vec<i32> = Vec::new();
  let mut result = 0;

  for n in vec {
    for i in &arr {
      let (max, min) = sorted_number(n, i.clone());
      if max % min == 0 {
        result = max / min;
        break;
      }
    }

    if result > 0 {
      break;
    }

    arr.push(n);
  }

  result
}

fn sorted_number(a: i32, b: i32) -> (i32, i32) {
  if a > b {
    (a, b)
  } else {
    (b, a)
  }
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
  input.lines().map(|line| {
    line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
  }).collect()
}
