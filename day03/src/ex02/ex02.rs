use std::fs;

use crate::ex01::ex01::create_schematic;
use crate::ex01::ex01::get_engine_part;
use crate::ex01::ex01::Node;

fn get_gear_parts(content: &Vec<Vec<Node>>, index_x: usize, index_y: usize) -> Vec<String> {
    let mut neighbours: Vec<&Node> = Vec::new();
    let max_x = content[0].len();
    let max_y = content.len();
    let is_y_overflow: bool = index_y.overflowing_sub(1).1;
    let is_x_overflow: bool = index_x.overflowing_sub(1).1;
    let mut gear_parts: Vec<String> = Vec::new();

    if !is_y_overflow && !is_x_overflow && index_x + 1 < max_x {
        neighbours.push(&content[index_y - 1][index_x]);
        neighbours.push(&content[index_y - 1][index_x - 1]);
        neighbours.push(&content[index_y - 1][index_x + 1]);
    } 
    if index_y + 1 < max_y && !is_x_overflow && index_x + 1 < max_x {
        neighbours.push(&content[index_y + 1][index_x]);
        neighbours.push(&content[index_y + 1][index_x - 1]);
        neighbours.push(&content[index_y + 1][index_x + 1]);
    }
    if !is_x_overflow {
        neighbours.push(&content[index_y][index_x - 1]);
    }
    if index_x + 1 < max_x {
        neighbours.push(&content[index_y][index_x + 1]);
    }

    for n in neighbours {
        if n.symbol.is_digit(10) {
            gear_parts.push(get_engine_part(n.position_x, &content[n.position_y]));
        }
    }
    gear_parts.dedup();
    return gear_parts;
}

fn create_gears(engine_schematic: Vec<Vec<Node>>) -> Vec<Vec<String>> {
    let mut gears: Vec<Vec<String>> = Vec::new();

    for line in &engine_schematic {
        for b in line {
            if b.symbol == '*' {
                let res = get_gear_parts(&engine_schematic, b.position_x, b.position_y);
                if res.len() == 2 {
                    gears.push(res);
                }
            }
        }
    }
    return gears;
}

pub fn calculate_gears_count(gears: Vec<Vec<String>>) -> i32 {
    let mut result: i32 = 0;
    
    for gear in gears {
        result += gear[0].parse::<i32>().unwrap() * gear[1].parse::<i32>().unwrap();
    }

    return result;
}

pub fn count_gear_ratios(filename: String) -> i32 {
    let content: Vec<String> = fs::read_to_string(filename)
        .expect("Error oppening file")
        .lines()
        .map(String::from)
        .collect();
    let engine_schematic: Vec<Vec<Node>> = create_schematic(content);
    let gears: Vec<Vec<String>> = create_gears(engine_schematic);
    
    return calculate_gears_count(gears);
}