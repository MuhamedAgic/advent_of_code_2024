use std::collections::HashMap;
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

fn blink(line: &str, n_times: i32) -> i64 {
    let stones: Vec<&str> = line.split_whitespace().collect();
    let mut stone_counts: HashMap<String, i64> = HashMap::new();

    // keep track of counts in hashmap
    for &stone in &stones {
        *stone_counts.entry(stone.to_string().clone()).or_insert(0) += 1;
    }

    for _ in 0..n_times {
        let mut new_counts = HashMap::new();
        for (stone, &count) in &stone_counts {
            let outcome = apply_rules(stone);
            if outcome.contains(' ') {
                let mut parts = outcome.split_whitespace();
                let first = parts.next().unwrap().to_string();
                let second = parts.next().unwrap().to_string();
                *new_counts.entry(first).or_default() += count;
                *new_counts.entry(second).or_default() += count;
            }
            else {
                *new_counts.entry(outcome).or_default() += count;
            }
        }
        stone_counts = new_counts;
    }
    stone_counts.values().sum()
}

fn part_one_and_two(input: &str, blink_count: i32) -> i64 {
    let stone_line = String::from(input);
    blink(&stone_line, blink_count)
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