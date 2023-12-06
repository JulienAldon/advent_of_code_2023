use std::fs;

pub struct Node {
    pub position_x: usize,
    pub position_y: usize,
    pub symbol: char
}

fn is_symbol(c: char) -> bool {
    if c.is_ascii_punctuation() && c != '.' {
        return true;
    }
    return false;
}

fn is_engine_part(content: &Vec<Vec<Node>>, index_x: usize, index_y: usize) -> bool {
    let mut neighbours: Vec<char> = Vec::new();
    let max_x = content[0].len();
    let max_y = content.len();
    let is_y_overflow: bool = index_y.overflowing_sub(1).1;
    let is_x_overflow: bool = index_x.overflowing_sub(1).1;

    if !content[index_y][index_x].symbol.is_digit(10) {
        return false;
    }

    if !is_y_overflow && !is_x_overflow && index_x + 1 < max_x {
        neighbours.push(content[index_y - 1][index_x].symbol);
        neighbours.push(content[index_y - 1][index_x - 1].symbol);
        neighbours.push(content[index_y - 1][index_x + 1].symbol);
    } 
    if index_y + 1 < max_y && !is_x_overflow && index_x + 1 < max_x {
        neighbours.push(content[index_y + 1][index_x].symbol);
        neighbours.push(content[index_y + 1][index_x - 1].symbol);
        neighbours.push(content[index_y + 1][index_x + 1].symbol);
    }
    if !is_x_overflow {
        neighbours.push(content[index_y][index_x - 1].symbol);
    }
    if index_x + 1 < max_x {
        neighbours.push(content[index_y][index_x + 1].symbol);
    }

    for n in neighbours {        
        if is_symbol(n) {
            return true;
        }
    }
    return false;
}

pub fn get_engine_part(position_x: usize, content: &Vec<Node>) -> String {
    let mut engine_part: String = "".to_string();
    let mut i = position_x;

    while i < content.len() && content[i].symbol.is_digit(10) {
        engine_part.push(content[i].symbol);
        i += 1;
    }
    i = position_x;
    while i >= 1 && content[i - 1].symbol.is_digit(10) {
        engine_part.insert(0, content[i - 1].symbol);
        i -= 1;
    }
    return engine_part;
}

pub fn create_schematic(content: Vec<String>) -> Vec<Vec<Node>> {
    let mut engine_schematic: Vec<Vec<Node>> = Vec::new();

    for (index_y, line) in content.iter().enumerate() {
        let mut engine_line = Vec::new();
        for (index_x, c) in line.chars().enumerate() {
            let node = Node {
                position_x: index_x,
                position_y: index_y,
                symbol: c
            };
            engine_line.push(node);
        }
        engine_schematic.push(engine_line);
    }

    return engine_schematic;
}

fn create_engine_parts(engine_schematic: Vec<Vec<Node>>) -> Vec<String> {
    let mut engine_parts: Vec<String> = Vec::new();
    
    for line in &engine_schematic {
        for node in line {
            let is_engine_part_bool = is_engine_part(&engine_schematic, node.position_x, node.position_y);
            if is_engine_part_bool {
                engine_parts.push(get_engine_part(node.position_x, &line));
            }
        }
    }
    return engine_parts;
}

fn calculate_engine_parts(engine_parts: Vec<String>) -> i32 {
    let mut tmp: String = "".to_string();
    let mut result: i32 = 0;

    for l in engine_parts {
        if l == tmp {
            continue
        }
        tmp = l.clone();
        result += l.parse::<i32>().unwrap();
    }
    return result;
}

pub fn count_engine_part_numbers(filename: String) -> i32 {
    let content: Vec<String> = fs::read_to_string(filename)
        .expect("Error oppening file")
        .lines()
        .map(String::from)
        .collect();

    let engine_schematic: Vec<Vec<Node>> = create_schematic(content);
    let engine_parts: Vec<String> = create_engine_parts(engine_schematic);
    return calculate_engine_parts(engine_parts);
}