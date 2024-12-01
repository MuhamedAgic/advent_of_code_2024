use std::time::SystemTime;
use itertools::Itertools;
use advent_of_code_2024::utils;

fn part_one(input: &str) -> i32 {
    let mut first_location_ids = Vec::new();
    let mut second_location_ids = Vec::new();

    for line in input.lines() {
        let (mut first, mut second): (&str, &str) = line.split("   ").collect_tuple().unwrap();
        let first = first.parse::<i32>().unwrap();
        let second = second.parse::<i32>().unwrap();
        first_location_ids.push(first);
        second_location_ids.push(second);
    }

    first_location_ids.sort_unstable();
    second_location_ids.sort_unstable();

    let mut total_distance = 0;
    for (first, second) in first_location_ids.iter().zip(second_location_ids.iter()) {
        let mut distance = second - first;
        total_distance += distance.abs();
    }

    total_distance
}

fn part_one_v2(input: &str) -> i32 {
    let mut first_location_ids = Vec::new();
    let mut second_location_ids = Vec::new();

    input
        .lines()
        .map(|line| line.split("   ").collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .for_each(|(a, b)| {
            first_location_ids.push(a);
            second_location_ids.push(b);
        });

    first_location_ids.sort_unstable();
    second_location_ids.sort_unstable();

    let mut total_distance = 0;
    for (first, second) in first_location_ids.iter().zip(second_location_ids.iter()) {
        let mut distance = second - first;
        total_distance += distance.abs();
    }
    
    first_location_ids
        .iter()
        .zip(second_location_ids.iter())
        .fold(0, |total_distance, (a, b)| total_distance + (a - b).abs());

    total_distance
}

fn part_two(input: &str) -> i32 {
    let mut first_location_ids = Vec::new();
    let mut second_location_ids = Vec::new();

    for line in input.lines() {
        let (mut first, mut second): (&str, &str) = line.split("   ").collect_tuple().unwrap();
        let first = first.parse::<i32>().unwrap();
        let second = second.parse::<i32>().unwrap();
        first_location_ids.push(first);
        second_location_ids.push(second);
    }

    let mut similarity_score = 0;
    for first in first_location_ids.iter() {
        let mut appearances = 0;
        for second in second_location_ids.iter() {
            if first == second {
                appearances += 1;
            }
        }
        similarity_score += first * appearances;
    }

    similarity_score
}

fn part_two_v2(input: &str) -> i32 {
    let mut first_location_ids = Vec::new();
    let mut second_location_ids = Vec::new();

    input
        .lines()
        .map(|line| line.split("   ").collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .for_each(|(a, b)| {
            first_location_ids.push(a);
            second_location_ids.push(b);
        });

    let mut similarity_score = 0;
    for first in first_location_ids.iter() {
        let mut appearances = 0;
        for second in second_location_ids.iter() {
            if first == second {
                appearances += 1;
            }
        }
        similarity_score += first * appearances;
    }

    similarity_score
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(1);

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    println!("Part One V2: {}", part_one_v2(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    println!("Part Two: {}", part_two(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    println!("Part Two V2: {}", part_two_v2(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    Ok(())

}
