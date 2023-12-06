use std::fs;

pub struct GameConfig {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

pub struct Game {
    pub id: i32,
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

fn get_game_id(game_str: String) -> i32 {
    let game = game_str.split(':').filter(|x| x.contains("Game")).collect::<String>();
    let game_id = game.split(" ").last().unwrap();
    let result = game_id.parse::<i32>().unwrap();
    return result;
}

pub fn get_game_content(game_str: String) -> Game {
    let mut green: String = "0".to_string();
    let mut red: String = "0".to_string();
    let mut blue: String = "0".to_string();
    let game = game_str.split(':').filter(|x| !x.contains("Game")).collect::<String>();
    let game_infos = game.split(';');
    for info in game_infos {
        for color in info.split(',') {
            if color.contains("green") {
                let num_g = color.split(" ").filter(|x| !x.contains("green")).collect::<String>();
                if num_g.parse::<i32>().unwrap() > green.parse::<i32>().unwrap() {
                    green = num_g;
                }
            }
            if color.contains("red") {
                let num_r = color.split(" ").filter(|x| !x.contains("red")).collect::<String>();
                if num_r.parse::<i32>().unwrap() > red.parse::<i32>().unwrap() {
                    red = num_r;
                }            
            }
            if color.contains("blue") {
                let num_b = color.split(" ").filter(|x| !x.contains("blue")).collect::<String>();
                if num_b.parse::<i32>().unwrap() > blue.parse::<i32>().unwrap() {
                    blue = num_b;
                }            
            }
        }
    }
    return Game {
        id: 0,
        red: red.parse::<i32>().unwrap(),
        green: green.parse::<i32>().unwrap(),
        blue: blue.parse::<i32>().unwrap()
    }
}

fn is_game_possible(game: &Game, game_config: &GameConfig) -> bool {
    return game.red <= game_config.red && game.green <= game_config.green && game.blue <= game_config.blue;
}

pub fn count_possible_games(filename: String, game_config: GameConfig) -> i32 {
    let content: Vec<String> = fs::read_to_string(filename)
        .expect("Error oppening file")
        .lines()
        .map(String::from)
        .collect();
    let mut result: i32 = 0;

    for line in content {
        let game_id = get_game_id(line.clone());
        let mut game = get_game_content(line.clone());
        game.id = game_id;
        if is_game_possible(&game, &game_config) {
            result += game.id;
        }
    }
    return result;
}