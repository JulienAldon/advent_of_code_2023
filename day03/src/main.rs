mod ex01 {
    pub mod ex01;
}

mod ex02 {
    pub mod ex02;
}

use ex01::ex01::count_engine_part_numbers;
use ex02::ex02::count_gear_ratios;

fn main() {
    println!("{}", count_engine_part_numbers("./ex01_input.txt".to_string()));
    println!("{}", count_gear_ratios("./ex02_input.txt".to_string()));
}
