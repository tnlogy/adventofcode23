use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join("input/input2.txt");
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    let mut sum2 = 0;

    // Define regular expressions for game and color sections
    let game_regex = Regex::new(r"Game (\d+): (.*)$").unwrap();
    let color_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    for line in reader.lines() {
        // Iterate over matches for each game
        for game_match in game_regex.captures_iter(&line?) {
            // Extract game number
            let game_number: u32 = game_match[1].parse().unwrap();
            let color_section = &game_match[2];
            // Extract and print color sections
            println!("Game {}: Colors: {}", game_number, color_section);
            let mut valid = true;
            let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);


            for color_strings in color_section.split(";") {
                for color_string in color_strings.split(",") {
                    for color_match in color_regex.captures_iter(color_string) {
                        let count: u32 = color_match[1].parse().unwrap();
                        let color = &color_match[2];
                        if color == "red" { min_red = min_red.max(count); }
                        if color == "green" { min_green = min_green.max(count); }
                        if color == "blue" { min_blue = min_blue.max(count); }
                        if color == "red" && count > 12 { valid = false; }
                        if color == "green" && count > 13 { valid = false; }
                        if color == "blue" && count > 14 { valid = false; }
                    }
                }
            }
            if valid { sum += game_number; }
            sum2 += min_red * min_green * min_blue;
        }
    }

    println!("Result 1: {}, 2: {}", sum, sum2);
    Ok(())
}
