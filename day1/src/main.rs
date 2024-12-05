use std::{collections::btree_map::Values, fs, ops::Sub};

/**
 * find 50 stars
 */
fn main() {
    let input = fs::read_to_string(
        "C:/Users/Gabriel Spinola/Desktop/projects/AdventOfCode/day1/src/input.txt",
    )
    .expect("Shoud have benn able to read the file");

    let listed = input.split_ascii_whitespace();
    let mut left: Vec<i32> = listed
        .clone()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, value)| value.parse::<i32>().expect("Failed to parte value"))
        .collect();
    let mut right: Vec<i32> = listed
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0)
        .map(|(_, value)| value.parse::<i32>().expect("Failed to parte value"))
        .collect();

    left.sort();
    right.sort();
    let mut right_iter = right.iter_mut();

    let distances: i32 = left
        .into_iter()
        .map(|value| (right_iter.next().unwrap().sub(value)).abs())
        .sum();

    print!("{}", distances)
}
