type Digits = [(&'static str, u32); 9];

pub fn two() {
    let input = include_str!("input");
    let digits: Digits = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
    ];
    let result: u32 = input
        .lines()
        .map(|line| {
            first_number(&line, &digits) * 10
            + last_number(&line, &digits)
        }).sum();

    println!("{result}");
}
                
fn first_number(line: &str, digits: &Digits) -> u32 {
    // Iterates through every character of a line
    for (index, character) in line.chars().enumerate() {
        // If it is a digit, return it
        if character.is_digit(10) { return character.to_digit(10).unwrap() }
        // If not, then checks if substring from beginning to that character
        // already makes up a word
        for digit in digits {
            if line[..=index].contains(digit.0) { return digit.1; }
        }
    }

    0
}

fn last_number(line: &str, digits: &Digits) -> u32 {
    for (index, character) in line.chars().rev().enumerate() {
        if character.is_digit(10) { return character.to_digit(10).unwrap(); }
        // Basically the name logic as in first_number() but checks substring
        // from character index (calculated as length minus index minus 1)
        // to the end
        for digit in digits {
            if line[(line.len() - index - 1)..].contains(digit.0) { return digit.1; }
        }
    }
    
    0 // this line is only needed to please the compiler
}