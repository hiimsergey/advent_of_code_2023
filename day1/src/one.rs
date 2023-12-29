pub fn one() {
    let input = include_str!("input");
    let result: u32 = input.lines().map(|line| {
        let (mut left, mut right) = (' ', ' ');

        for c in line.chars() {
            if c.is_digit(10) {
                if left == ' ' { left = c; }
                right = c;
            }
        }

        left.to_digit(10).unwrap() * 10
        + right.to_digit(10).unwrap()
    }).sum();

    println!("{result}");
}
