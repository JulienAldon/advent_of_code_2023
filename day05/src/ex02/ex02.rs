use std::fs;
use crate::ex01::ex01::Range;
use crate::ex01::ex01::init_garden;
use crate::ex01::ex01::translate_all_seeds;

fn push_n_elements(vector: &mut Vec<i64>, start: i64, n: i64) {
    for i in start..start+n {
        vector.push(i);
    }
}

fn init_seeds(seeds_str: String) -> Vec<i64> {
    let seeds: Vec<i64> = seeds_str.split(' ').filter(|x| *x != "").map(|x| x.parse::<i64>().unwrap()).collect();
    let mut i = 0;
    let mut all_seeds: Vec<i64> = Vec::new();
    for _seed in seeds.iter().step_by(2) {
        println!("{:?}, {}", seeds[i], seeds[i+1]);
        push_n_elements(&mut all_seeds, seeds[i], seeds[i+1]);
        i+=2;
    }
    // println!("{:?}", seeds);
    return all_seeds;
}

pub fn find_nearest_seed_range_location(filename: String) -> i64 {
    let content: Vec<String> = fs::read_to_string(filename)
        .expect("Error oppening file")
        .lines()
        .map(String::from)
        .collect();

    let seeds_str: String = content[0].split(':').filter(|x| !x.contains("seeds")).collect();
    let seeds: Vec<i64> = init_seeds(seeds_str);
    
    let all_maps: Vec<Vec<Range>> = init_garden(content);
    let all_positions: Vec<i64> = translate_all_seeds(seeds, all_maps);
    
    return *all_positions.iter().min().unwrap();
}