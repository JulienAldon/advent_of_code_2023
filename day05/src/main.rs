mod ex01 {
    pub mod ex01;
}

mod ex02 {
    pub mod ex02;
}

use crate::ex01::ex01::find_nearest_seed_location;
use crate::ex02::ex02::find_nearest_seed_range_location;

fn main() {
    println!("{}", find_nearest_seed_location("./ex01_input.txt".to_string()));
    println!("{}", find_nearest_seed_range_location("./ex02_input.txt".to_string()));
}
