mod ex01 {
    pub mod ex01;
}

mod ex02 {
    pub mod ex02;
}

use ex01::ex01::count_winning_games;
use ex02::ex02::count_process_scratchcards;

fn main() {
    println!("{}", count_winning_games("./ex01_input.txt".to_string()));
    println!("{}", count_process_scratchcards("./ex02_input.txt".to_string()));
}
