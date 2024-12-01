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

fn main() {
    let input = utils::read_input(1);
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}
