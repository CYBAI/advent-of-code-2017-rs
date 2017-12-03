//!
//! Implementation of Day 1 in Advent of Code 2017
//!

pub fn parse_sequence(input: &str) -> i32 {
    let mut total: i32 = 0;
    let chars: Vec<char> = input.chars().collect();

    let len = chars.len() - 1;

    for (idx, each_char) in input.chars().enumerate() {
        let check_idx: usize = if idx == len {
            0
        } else {
            idx + 1
        };

        if each_char == chars[check_idx] {
            if let Some(int) = each_char.to_digit(10) {
                total += int as i32;
            }
        }
    }

    total
}

pub fn halfway_around(input: &str) -> i32 {
    let mut total: i32 = 0;
    let chars: Vec<char> = input.chars().collect();

    let half = chars.len() / 2;

    for (idx, each_char) in input.chars().enumerate() {
        if idx == half {
            break;
        }

        if each_char == chars[half + idx] {
            if let Some(int) = each_char.to_digit(10) {
                total += int as i32;
            }
        }
    }

    total * 2
}