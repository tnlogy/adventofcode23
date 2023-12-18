use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

fn scratch(input: &str, copies: &mut HashMap<u32, u32>) -> Option<u32> {
    let card_regex = Regex::new(r"Card\s+(\d+): (.*)$").unwrap();
    if let Some(card) = card_regex.captures(input) {
        //println!("m1");
        let (winners_str, numbers_str) = card[2].split_once('|').unwrap();
        //println!("w {} n {}", &winners_str, &numbers_str);
        let match_nr: u32 = (&card[1]).parse().unwrap();
        let winners: Vec<u32> = winners_str.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let numbers: Vec<u32> = numbers_str.split_whitespace().map(|s| s.parse().unwrap()).collect();

        let winning_numbers: Vec<u32> = numbers.iter().filter(|n| winners.contains(&n)).cloned().collect();
        let nr_copies = *copies.entry(match_nr).or_insert(1);

        if winning_numbers.len() > 0 {
            let value = u32::pow(2, (winning_numbers.len() - 1) as u32);
            for i in match_nr+1..=match_nr+(winning_numbers.len() as u32) {
                *copies.entry(i).or_insert(1) += nr_copies;
            }

            println!("match {}: {} wn: {}", match_nr, value, winning_numbers.iter().map(ToString::to_string).collect::<Vec<String>>().join(" "));
            return Some(value);
        } else {
             println!("match {}: none", match_nr);
        }
    }
    Some(0)
}

fn main() -> io::Result<()> {
    let mut copies: HashMap<u32, u32> = HashMap::new();
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join("input/input4.txt");
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let sum: u32 = reader.lines()
        .filter_map(|line| line.ok().and_then(|text| scratch(&text, &mut copies)))
        .sum();

    println!("Result: {}", sum);

    /*let mut sum2 = 0;
    for (_match_nr, copy_nr) in copies.iter() {
        sum2 += copy_nr;
    }*/
    // more compact version
    let sum2 = copies.iter().map(|(_match_nr, copy_nr)| copy_nr).sum::<u32>();
    println!("Result 2: {}", sum2);

    Ok(())
}
