use crate::one::get_games_vec;

pub fn two() {
    let input = include_str!("input");
    let result: u32 = input.lines().map(|line| get_game_power(line)).sum();

    println!("{result}");
}

fn get_game_power(game: &str) -> u32 {
    let mut min: (u32, u32, u32) = (0, 0, 0); // (red, green, blue)

    for num_color_tuple in get_games_vec(game) {
        match num_color_tuple {
            (value, "red") => if value > min.0 { min.0 = value },
            (value, "green") => if value > min.1 { min.1 = value },
            (value, /* "blue" */ _) => if value > min.2 { min.2 = value }
        }
    }

    min.0 * min.1 * min.2
}