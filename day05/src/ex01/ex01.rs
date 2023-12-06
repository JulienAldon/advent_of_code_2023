use std::fs;

pub struct Range {
    pub src: i64,
    pub dest: i64,
    pub length: i64,
}

pub fn init_ranges(map_string: Vec<&String>) -> Vec<Range> {
    let mut ranges = Vec::new();

    for elem in map_string {
        let elems: Vec<i64> = elem.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let range = Range {
            src: elems[1],
            dest: elems[0],
            length: elems[2]
        };
        ranges.push(range);
    }
    return ranges
}

pub fn find_next_map(index: usize, content: Vec<String>) -> Vec<String> {
    let mut i = index;
    let mut result = Vec::new();

    while i < content.len() && content[i] != "".to_string() {
        result.push(content[i].clone());
        i+=1;
    }
    return result;
}

pub fn init_garden(content: Vec<String>) -> Vec<Vec<Range>> {
    let mut pos = 0;
    let mut all_maps: Vec<Vec<Range>> = Vec::new();

    for i in 2..9 {
        let current_map_string: Vec<String> = find_next_map(pos+i, content.clone());
        let current_map: Vec<&String> = current_map_string.iter().filter(|x| !x.contains(':')).collect();
        all_maps.push(init_ranges(current_map));
        pos+= current_map_string.len();
    }
    return all_maps;
}

pub fn translate_seed(map: &Vec<Range>, seed: i64) -> i64 {
    for range in map {
        if seed >= range.src && seed <= range.src + range.length {
            let offset = range.dest - range.src;
            return seed + offset;
        }
    }
    return seed;
}

pub fn translate_all_seeds(seeds: Vec<i64>, all_maps: Vec<Vec<Range>>) -> Vec<i64> {
    let mut all_positions: Vec<i64> = Vec::new();

    for seed in seeds {
        let mut tmp = seed;
        
        for map in &all_maps {
            tmp = translate_seed(map, tmp);
        }
        all_positions.push(tmp);
    }
    return all_positions;
}

pub fn find_nearest_seed_location(filename: String) -> i64 {
    let content: Vec<String> = fs::read_to_string(filename)
        .expect("Error oppening file")
        .lines()
        .map(String::from)
        .collect();
    
    let seeds_str: String = content[0].split(':').filter(|x| !x.contains("seeds")).collect();
    let seeds: Vec<i64> = seeds_str.split(' ').filter(|x| *x != "").map(|x| x.parse::<i64>().unwrap()).collect();
   
    let all_maps: Vec<Vec<Range>> = init_garden(content);
    let all_positions: Vec<i64> = translate_all_seeds(seeds, all_maps);
    
    return *all_positions.iter().min().unwrap();
}