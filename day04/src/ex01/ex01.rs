use std::fs;

pub fn get_cards(line: String) -> String {
    return line.split(':').filter(|x| !x.contains("Card")).collect::<String>();

}

pub fn get_game(cards: String) -> Option<(String, String)> {
    let winning_nbs: String = match cards.split('|').nth(0) {
        Some(x) => {x.to_string()},
        None => {"None".to_string()}
    };
    if winning_nbs == "None" {
        println!("Error: no winning nbs found, input incorectly formed");
        return None;
    } 
    let game_nbs = match cards.split('|').last() {
        Some(x) => {x.to_string()},
        None => {"None".to_string()}
    };
    if game_nbs == "None" {
        println!("Error: no winning nbs found, input incorectly formed");
        return None;
    } 
    return Some((winning_nbs, game_nbs));
}

pub fn find_win_list(winning_nbs: String, game_nbs: String) -> Vec<String> {
    let mut game_list: Vec<String> = Vec::new();
    let game: Vec<&str> = game_nbs.split(' ').filter(|x| *x != "").collect::<Vec<&str>>();
    let table = winning_nbs.split(' ').filter(|x| *x != "");

    for n in table {
        if game.contains(&n) {
            game_list.push(n.to_string());
        }
    }
    return game_list;
}

pub fn count_winning_games(filename: String) -> i32 {
    let content: Vec<String> = fs::read_to_string(filename)
        .expect("Error oppening file")
        .lines()
        .map(String::from)
        .collect();

    let mut result:i32 = 0;

    for line in content {
        let cards = get_cards(line.clone());
        let (winning_nbs, game_nbs) = get_game(cards).unwrap();
        let win_list = find_win_list(winning_nbs, game_nbs);

        if win_list.len() >= 1 {
            result += 2_i32.pow((win_list.len() as i32 - 1) as u32);
        }
    }
    return result;
}

// 26218