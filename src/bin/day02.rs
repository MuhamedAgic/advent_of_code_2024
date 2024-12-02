use std::cmp::Ordering;
use std::time::SystemTime;
use itertools::Itertools;
use advent_of_code_2024::utils;
use num_bigint::{BigInt, Sign};

fn is_sorted_ascending<T>(data: &Vec<T>) -> Option<bool>
    where T: Ord {
    if data.is_empty() {
        return None;
    }
    // take windows of 2 and check if every item after the current is greater or equal than current
    return Some(data.windows(2).all(|w| w[0] <= w[1]));
}

fn is_sorted_descending<T>(data: &Vec<T>) -> Option<bool>
where T: Ord {
    if data.is_empty() {
        return None;
    }
    // take windows of 2 and check if every item after the current is greater or equal than current
    return Some(data.windows(2).all(|w| w[0] >= w[1]));
}

fn is_valid_next_level(first: i32, second: i32) -> bool {
    return ((second - first).abs() >= 1) && ((second - first).abs() <= 3);
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    if numbers.is_empty() {
        return false;
    }
    let mut prev = numbers[0];
    for (i, curr) in numbers.iter().enumerate() {
        if i == 0 {
            prev = *curr;
            continue;
        }
        if !is_valid_next_level(prev, *curr) {
            return false;
        }
        prev = *curr;
    }
    return true;
}

fn is_valid(numbers: &Vec<i32>) -> bool {
    if !numbers.iter().duplicates().collect_vec().is_empty() {
        // no duplicates allowed, only strictly ascending or descending
        return false;
    }

    let mut ordering = Ordering::Equal;
    if is_sorted_ascending(&numbers).expect("oh no") {
        ordering = Ordering::Greater;
    }
    else if is_sorted_descending(&numbers).expect("oh no") {
        ordering = Ordering::Less;
    }
    else {
        return false;
    }

    match ordering {
        Ordering::Greater => is_safe(&numbers),
        Ordering::Less => is_safe(&numbers),
        Ordering::Equal => false
    }
}

fn part_one(input: &str) -> i32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let mut is_valid_row = true;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if !numbers.iter().duplicates().collect_vec().is_empty() {
            // no duplicates allowed, only strictly ascending or descending
            continue;
        }

        let mut ordering = Ordering::Equal;
        if is_sorted_ascending(&numbers).expect("oh no") {
            ordering = Ordering::Greater;
        }
        else if is_sorted_descending(&numbers).expect("oh no") {
            ordering = Ordering::Less;
        }
        else {
            continue;
        }

        match ordering {
            Ordering::Greater => is_valid_row = is_safe(&numbers),
            Ordering::Less => is_valid_row = is_safe(&numbers),
            Ordering::Equal => println!("how?")
        }

        if is_valid_row {
            safe_count += 1;
            println!("Safe row: {:?}", numbers);
        }
    }
    safe_count
}

fn part_two(input: &str) -> i32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let mut is_valid_row = true;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if is_valid(&numbers) {
            safe_count += 1;
            println!("Safe row: {:?}", numbers);
        }
        else {
            let mut valid_combi = false;
            for combi in numbers.iter().copied().combinations(numbers.len() - 1) {
                // println!("Combi: {:?}", combi);
                if is_valid(&combi) {
                   valid_combi = true;
                }
            }
            if valid_combi {
                safe_count += 1;
                println!("Safe row AFTER correction: {:?}", numbers);
            }
        }
    }
    safe_count
}


fn project_euler_48() {
    let mut result = BigInt::new(Sign::Plus, vec![0]);;
    for i in 1..=1000 {
        let mut num: Vec<_> = vec![i];
        let x = BigInt::new(Sign::Plus, num);
        let res = num::pow(x, i as usize);
        result += res;
    }
    println!("{}", result);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(2);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day02.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    println!("Part two: {}", part_two(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    Ok(())
}