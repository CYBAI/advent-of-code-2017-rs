//!
//! Implementation of Day 1 in Advent of Code 2017
//!

pub fn parse_sequence(input: &str) -> i32 {
    let mut total: i32 = 0;
    let chars: Vec<&str> = input.split("").collect();
    let len = chars.len() - 3;
    let last_empty = chars.len() - 1;
    let new_line_idx = chars.len() - 2;

    for (idx, each_char) in input.split("").into_iter().enumerate() {
        if idx == 0 || idx == last_empty || idx == new_line_idx {
            continue;
        }

        let check_idx: usize = if idx == len {
            1
        } else {
            idx + 1
        };

        if each_char == chars[check_idx] {
            if let Ok(int) = each_char.parse::<i32>() {
                total += int;
            }
        }
    }

    total
}