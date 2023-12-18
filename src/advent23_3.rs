use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

fn get_map(map: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
    if y < map.len() {
        if x < map[y].len() {
            return Some(map[y][x]);
        } else {
            return Some('.');
        }
    }
    None
}

fn get_value(map: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let v = get_map(&map, x, y).unwrap().to_digit(10).unwrap();
    if x > 0 {
        if let Some(left) = get_map(&map, x - 1, y) {
            if left.is_digit(10) {
                return v + 10 * get_value(&map, x - 1, y);
            }
        }
    }
    v
}

fn close_to_part(map: &Vec<Vec<char>>, x: usize, y: usize, len: usize) -> Option<(char, usize, usize)> {
    for yp in y.saturating_sub(1)..=y + 1 {
        for xp in x.saturating_sub(len)..=x + 1 {
            if let Some(v) = get_map(&map, xp, yp) {
                if !v.is_digit(10) && !v.eq(&'.') {
                    return Some((v, xp, yp));
                }
            }
        }
    }
    None
}

fn get_part_nr(map: &Vec<Vec<char>>, x: usize, y: usize) -> Option<(u32, char, usize, usize)> {
    if let (Some(v), Some(r)) = (get_map(&map, x, y), get_map(&map, x + 1, y)) {
        if v.is_digit(10) && !r.is_digit(10) {
            let value = get_value(&map, x, y);
            let valueLen = value.to_string().len();
            if let Some((part, x, y)) = close_to_part(&map, x, y, valueLen) {
                return Some((value, part, x, y));
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join("input/input3.txt");
    let map = read_map(&file_path)?;
    let mut gears = HashMap::new();
    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Some((v, part, px, py)) = get_part_nr(&map, x, y) {
                if part.eq(&'*') {
                    let mut entry = gears.entry(format!("{}-{}", px, py)).or_insert(Vec::new());
                    entry.push(v);
                }
                sum += v;
            }
        }
    }

    println!("Result {}", sum);

    // sum the values of the gears '*'
    let mut sum2 = 0;
    for (key, values) in gears.iter() {
        if values.len() == 2 {
            let m = values.iter().fold(1, |acc, x| acc * x);
            sum2 += m;
        }
    }

    println!("Result2 {}", sum2);
    Ok(())
}

fn read_map(file_path: &PathBuf) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut map = Vec::new();

    for line in reader.lines() {
        let row: Vec<char> = line?.chars().collect();
        map.push(row);
    }

    Ok(map)
}
