//!
//! Implementation of Day 1 in Advent of Code 2017
//!

pub fn parse_sequence(input: &str) -> i32 {
    let mut total: i32 = 0;
    let chars: Vec<char> = input.chars().collect();

    let len = chars.len() - 2;
    let new_line_idx = chars.len() - 1;

    for (idx, each_char) in input.chars().enumerate() {
        if idx == new_line_idx {
            continue;
        }

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