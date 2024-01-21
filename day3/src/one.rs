use std::collections::{HashMap, hash_map::Entry};

const INPUT: &str = include_str!("input");

struct GridNumber {
    index_end: usize,
    value: u32
}

pub fn one() {
    let all_digits: Vec<(usize, u32)> = get_all_digits();
    let all_numbers: HashMap<usize, GridNumber> = get_all_numbers(&all_digits);
    let result: u32 = all_numbers
        .iter()
        .filter(|(i, gn)| {
            // Goes through every digit of a number to check whether it's surrounded
            // by a special character.
            for index in **i..=gn.index_end {
                if is_adjacent(index) { return true; }
            }

            false
        })
        .map(|(_, gn)| { gn.value })
        .sum();
    // TODO TEST
    // Builds a vector of all the linear indices of all the adjacent numbers' first
    // digits.
    /*
    let mut res_vec: Vec<usize> = all_numbers
        .iter()
        .filter(|(i, gn)| {
            for index in **i..=gn.index_end {
                if is_adjacent(index) { return true; }
            }

            false
        })
        .map(|(i, _)| *i)
        .collect();
    res_vec.sort();
    for i in res_vec { print!("{}:{}\t\t", i / 141 + 1, i % 141 + 1); }
    */

    println!("{result}");
}

// Returns a vector of all the digits in the puzzle input (f32) with the corresponding
// linear index.
fn get_all_digits() -> Vec<(usize, u32)> {
    INPUT
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_numeric())
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
        .collect()
}

// Returns a Hashmap describing all numbers in the puzzle input.
// The key is the linear index of the first digit and GridNumber stores the overall
// value and the linear index of the last digit.
fn get_all_numbers(all_digits: &[(usize, u32)]) -> HashMap<usize, GridNumber> {
    let mut result: HashMap<usize, GridNumber> = HashMap::new();

    for (i, d) in all_digits {
        // Checks if the previous character is also a digit and thus part of the
        // current number.
        if let Entry::Occupied(mut entry) = result.entry(i - 1) {
            let grid_number = entry.get_mut();
            grid_number.index_end = *i;
            grid_number.value = grid_number.value * 10 + d;

            continue;
        }
        // Checks the character two indices back.
        if let Entry::Occupied(mut entry) = result.entry(i - 2) {
            if INPUT.chars().nth(i - 1).unwrap().is_numeric() {
                let grid_number = entry.get_mut();
                grid_number.index_end = *i;
                grid_number.value = grid_number.value * 10 + d;
            }

            continue;
        }
        result.insert(*i, GridNumber {
            index_end: *i,
            value: *d
        });
    }

    result
}

// Given a linear index, checks whether the digit is surrounded by a special character.
// Checks the eight adjacent positions relative to the linear index.
fn is_adjacent(index: usize) -> bool {
    for i in [-142, -141, -140, -1, 1, 140, 141, 142] {
        // Checks if this adjacent character exists (doesn't apply to edge digits).
        if -i > index as i16 { continue; }
        if let Some(c) = INPUT.chars().nth((index as i16 + i) as usize) {
            if !['.', '\n'].contains(&c) && !c.is_numeric() { return true; }
        }
    }

    false
}