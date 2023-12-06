use std::fs;

use crate::ex01::ex01::get_game_content;

pub fn count_minimum_cubes(filename: String) -> i32 {
    let content: Vec<String> = fs::read_to_string(filename)
        .expect("Error oppening file")
        .lines()
        .map(String::from)
        .collect();

    let mut result: i32 = 0;

    for line in content {
        let game = get_game_content(line.clone());
        result += game.red * game.green * game.blue;
    }
    
    return result;
}