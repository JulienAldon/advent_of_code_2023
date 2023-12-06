use std::fs;

use crate::ex01::ex01::get_cards;
use crate::ex01::ex01::get_game;
use crate::ex01::ex01::find_win_list;

pub fn count_process_scratchcards(filename: String) -> i32 {
    let content: Vec<String> = fs::read_to_string(filename)
        .expect("Error oppening file")
        .lines()
        .map(String::from)
        .collect();
    let mut c: Vec<i32> = Vec::new();

    for _a in &content {
        c.push(1);
    }
    c.push(1);

    let mut i = 0;

    for line in content {
        let cards = get_cards(line.clone());
        let (winning_nbs, game_nbs) = get_game(cards).unwrap();
        let win = find_win_list(winning_nbs, game_nbs);
        for (j, _n) in win.iter().enumerate() {
            c[j+i+1] = c[j+i+1] + c[i];
        }
        i+=1;
    }
    return c[0..i].iter().sum::<i32>();
}