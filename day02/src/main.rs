mod ex01 {
    pub mod ex01;
}

mod ex02 {
    pub mod ex02;
}

use ex01::ex01::count_possible_games;
use ex02::ex02::count_minimum_cubes;
use ex01::ex01::GameConfig;

fn main() {
    let new_game_config: GameConfig = GameConfig {
        red: 12,
        green: 13,
        blue: 14
    };

    println!("{}", count_possible_games("./ex01_input.txt".to_string(), new_game_config));
    println!("{}", count_minimum_cubes("./ex02_input.txt".to_string()));
}
