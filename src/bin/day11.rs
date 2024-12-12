use std::ops::Deref;
use std::time::SystemTime;
use regex::Regex;
use advent_of_code_2024::utils;

fn split_stone(stone: &str) -> String {
    let first_half = &stone[0..stone.len() / 2].trim_start_matches("0");
    let mut second_half = &stone[stone.len() / 2..];
    if second_half.chars().all(|c| c == '0') {
        second_half = "0";
    }
    else if second_half.len() > 1 {
        second_half = second_half.trim_start_matches("0");
    }
    return format!("{} {}", first_half, second_half);
}

fn apply_rules(stone: &str) -> String {
    if stone == "0" {
         return "1".to_string();
    }
    else if stone.len() % 2 == 0 {
        return split_stone(stone);
    }
    else {
        return (stone.parse::<i128>().unwrap() * 2024).to_string();
    }
}

fn blink(line: &str) -> String {
    let stones: Vec<&str> = line.split_whitespace().collect();
    let mut str_after_applied_rules = String::from("");
    for number in stones {
        let stone_outcome = apply_rules(number) + " ";
        str_after_applied_rules.push_str(&stone_outcome);
    }
    // println!("{}", str_after_applied_rules);
    return str_after_applied_rules;
}

fn part_one_and_two(input: &str, blink_count: i32) -> i32 {
    let mut stone_line = String::from(input);
    for i in 0..blink_count {
        stone_line = blink(&stone_line);
    }
    return stone_line.split_whitespace().count() as i32;
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(11);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day11.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one_and_two(&input, 25));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    println!("Part Two: {}", part_one_and_two(&input, 75));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    Ok(())
}