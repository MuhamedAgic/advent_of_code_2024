use std::time::{Duration, SystemTime};
use itertools::Itertools;
use advent_of_code_2024::utils;

fn get_most_frequent_element(vec: Vec<u8>) -> u8 {
    return **vec
        .iter()
        .counts() // get counts (n amount zeros, n amount ones
        .iter()
        .max_by_key(|entry| entry.1) // get max (zeros or ones)
        .unwrap().0; // get count
}

fn part_one(input: &String) -> i32 {
    let mut bytes = Vec::new();
    for line in input.lines() {
        let bit_count = line.len();
        let mut byte = vec![0; bit_count];
        for (i, char) in line.chars().enumerate() {
            if let Some(bit) = char.to_digit(10) {
                byte[i] = bit as u8;
            }
            else {
                panic!("Char could not be converted into a digit!");
            }
        }
        bytes.push(byte);
    }

    let bit_count = bytes[0].len();
    let mut most_common_bits = Vec::new();
    for i in 0..bit_count {
        let mut ith_bits = Vec::new();
        for byte in &bytes {
            ith_bits.push(byte[i]);
        }
        let most_common_bit = get_most_frequent_element(ith_bits);
        most_common_bits.push(most_common_bit);
    }

    let gamma_bits = most_common_bits;
    let elipson_bits = gamma_bits
        .iter()
        .map(|bit| if *bit == 0 { 1 } else { 0 })
        .collect_vec();

    let mut gamma_rate = 0;
    let mut elipson_rate = 0;

    for (i, bit) in gamma_bits.iter().enumerate() {
        let exp = bit_count - 1 - i;
        gamma_rate += 2_i32.pow(exp as u32) * *bit as i32;
    }
    for (i, bit) in elipson_bits.iter().enumerate() {
        let exp = bit_count - 1 - i;
        elipson_rate += 2_i32.pow(exp as u32) * *bit as i32;
    }

    let power_consumption = gamma_rate * elipson_rate;

    return power_consumption;
}

fn main() {
    // let input = utils::read_input_from_path("example_input/day3_2021.txt");
    let input = utils::read_input_from_path("input/day3_2021.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    match now.elapsed() {
        Ok(elapsed) => println!("Seconds elapsed {} ({} milliseconds, {} microseconds)", elapsed.as_secs(), elapsed.as_millis(), elapsed.as_micros()),
        Err(e) => println!("Error: {e:?}")
    }
    // println!("Part Two: {}", part_two(&input));
}