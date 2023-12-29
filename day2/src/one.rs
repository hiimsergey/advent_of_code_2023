pub fn one() {
    let input = include_str!("input");
    let result: u32 = input.lines().enumerate().map(|(index, game)| {
        // Every line (representing one game) is checked for validity
        if is_game_possible(game) { 1 + index as u32 } else { 0 as u32 }
    }).sum();

    println!("{result}");
}

fn is_game_possible(game: &str) -> bool {
    for num_color_tuple in get_games_vec(game) {
        // If the number (number_color_tuple.0) is is larger than the given
        // number of cubes (calculated by this match statement), the game can't
        // be possible
        if num_color_tuple.0 > {
            match num_color_tuple.1 {
                "red" => 12,
                "green" => 13,
                /* "blue" */ _ => 14
            }
        } { return false; }
    }
    
    true
}

// A vector of the combinations of number and color for this game is built
// starting from the position of the colon + 2 to skip the "Game: " part
pub fn get_games_vec(game: &str) -> Vec<(u32, &str)> {
    return game[game.chars().position(|x| x == ':').unwrap() + 2..]
        // A single collection is built by splitting by commas and semicolons
        .split(|c: char| c.is_ascii_punctuation())
        .map(|num_color_str| {
            let num_color_vec: Vec<&str> = num_color_str.split_whitespace().collect();
            // Returns a tuple (number of cubes, color) 
            (num_color_vec[0].parse().unwrap(), num_color_vec[1])
        })
        .collect();
}