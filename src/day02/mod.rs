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
  let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
  let mut result = 0;

  for n in vec {
    let f = factors(n);

    for (k, v) in map.iter() {
      if f.intersection(&v).count() >= cmp::min(v.len(), f.len()) {
        result = if k.clone() > n {
          k / n
        } else {
          n / k
        };

        break;
      }
    }

    if result > 0 {
      break;
    }

    map.insert(n, f.clone());
  }

  result
}

fn factors(num: i32) -> HashSet<i32> {
  let mut v: HashSet<i32> = HashSet::new();

  let mid = (num / 2 as i32) + 1;

  for i in 1..mid {
    if num % i == 0 {
      v.insert(num / i);
      v.insert(i);
    }
  }

  v
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
  input.lines().map(|line| {
    line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
  }).collect()
}
