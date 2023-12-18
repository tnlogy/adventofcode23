mod advent23_4;

use std::io::{self, BufRead};

static NUMBERS: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_digit(input: &str) -> Option<u32> {
    if let Some(c) = input.chars().next() {
        if c.is_digit(10) {
            return c.to_digit(10);
        }
    }

    for i in 0..NUMBERS.len() {
        if input.starts_with(NUMBERS[i]) {
            return Some((i + 1) as u32);
        }
    }
    None
}

fn decode(input: &str) -> Option<u32> {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;
    let len = input.len();

    for i in 0..len {
        let slice = &input[i..];
        if let Some(v) = get_digit(slice) {
            if !first.is_some() {
                first = Some(v);
                break;
            }
        }
    }
    for i in 0..len {
        let slice2 = &input[len - i - 1..];
        if let Some(v) = get_digit(slice2) {
            if !last.is_some() {
                last = Some(v);
                break;
            }
        }
    }

    if let (Some(a), Some(b)) = (first, last) {
        return Some(a * 10 + b);
    } else {
        return None;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;

    let sum: u32 = stdin.lock().lines()
        .filter_map(|line| line.ok().and_then(|text| decode(&text)))
        .sum();

    /*
        for line in stdin.lock().lines() {
            if let Ok(text) = line {
                if let Some(result) = decode(&text) {
                    sum += result;
                }
            }
        }*/
    println!("sum {}", sum);
}
