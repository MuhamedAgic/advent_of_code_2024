use std::time::{Duration, SystemTime};
use itertools::Itertools;
use advent_of_code_2024::utils;

fn get_most_frequent_element(vec: &Vec<u8>) -> u8 {
    return **vec
        .iter()
        .counts() // get counts (n amount zeros, n amount ones
        .iter()
        .max_by_key(|entry| entry.1) // get max (zeros or ones)
        .unwrap().0; // get count
}

fn get_most_frequent_element_favor(vec: &Vec<u8>, favor: u8) -> u8 {
    return *vec
        .iter()
        .counts() // get counts (n amount zeros, n amount ones), hashmap
        .into_iter()
        .max_by(|(k1, v1), (k2, v2)| {
            v1.cmp(v2).then_with(|| k1.cmp(k2))
        })
        .map(|(k, _)| k)
        .unwrap_or(&favor);
}

fn to_decimal(bits: &Vec<u8>) -> i32 {
    let mut decimal_value = 0;
    let bit_count = bits.len();
    for (i, bit) in bits.iter().enumerate() {
        let exp = bit_count - 1 - i;
        decimal_value += 2_i32.pow(exp as u32) * *bit as i32;
    }
    return decimal_value;
}

fn collect_bytes(input: &String) -> Vec<Vec<u8>> {
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
    return bytes;
}

fn get_most_common_bits(bytes: &Vec<Vec<u8>>) -> Vec<u8> {
    let bit_count = bytes[0].len();
    let mut most_common_bits = Vec::new();
    for i in 0..bit_count {
        let mut ith_bits = Vec::new();
        for byte in bytes {
            ith_bits.push(byte[i]);
        }
        let most_common_bit = get_most_frequent_element(&ith_bits);
        most_common_bits.push(most_common_bit);
    }
    return most_common_bits;
}

fn get_most_common_bits_part_two(bytes: &Vec<Vec<u8>>, favor: u8) -> Vec<u8> {
    let bit_count = bytes[0].len();
    let mut most_common_bits = Vec::new();
    for i in 0..bit_count {
        let mut ith_bits = Vec::new();
        for byte in bytes {
            ith_bits.push(byte[i]);
        }
        let most_common_bit = get_most_frequent_element_favor(&ith_bits, favor);
        most_common_bits.push(most_common_bit);
    }
    return most_common_bits;
}

fn part_one(input: &String) -> i32 {
    let bytes = collect_bytes(input);
    let most_common_bits = get_most_common_bits(&bytes);

    let gamma_bits = most_common_bits;
    let elipson_bits = gamma_bits
        .iter()
        .map(|bit| if *bit == 0 { 1 } else { 0 })
        .collect_vec();

    let gamma_rate = to_decimal(&gamma_bits);
    let elipson_rate = to_decimal(&elipson_bits);
    let power_consumption = gamma_rate * elipson_rate;

    return power_consumption;
}

fn part_two(input: &String) -> i32 {
    let bytes = collect_bytes(input);
    let most_common_bits = get_most_common_bits(&bytes);

    let mut oxygen_generator_rating_bytes = bytes.clone();
    let mut oxygen_generator_rating = 0;
    for i in 0..=most_common_bits.len() {
        if oxygen_generator_rating_bytes.len() == 1 {
            oxygen_generator_rating = to_decimal(&oxygen_generator_rating_bytes[0]);
            // println!("Final oxygen byte: {:?}", oxygen_generator_rating_bytes);
            // println!("Final oxygen rating: {}\n", oxygen_generator_rating);
            break;
        }
        let most_common_bits_oxygen_bytes = get_most_common_bits_part_two(&oxygen_generator_rating_bytes, 1);
        oxygen_generator_rating_bytes = oxygen_generator_rating_bytes
            .iter()
            .map(|byte| byte.clone())
            .filter(|byte| byte[i] == most_common_bits_oxygen_bytes[i])
            .collect();
        // println!("Current oxygen bytes: {:?}", oxygen_generator_rating_bytes);
    }

    let mut co2_scrubber_rating_bytes = bytes.clone();
    let mut co2_scrubber_rating = 0;
    for i in 0..=most_common_bits.len() {
        if co2_scrubber_rating_bytes.len() == 1 {
            co2_scrubber_rating = to_decimal(&co2_scrubber_rating_bytes[0]);
            // println!("Final co2 byte: {:?}", co2_scrubber_rating_bytes);
            // println!("Final co2 rating: {}\n", co2_scrubber_rating);
            break;
        }
        let most_common_bits_co2_bytes = get_most_common_bits_part_two(&co2_scrubber_rating_bytes, 0);
        co2_scrubber_rating_bytes = co2_scrubber_rating_bytes
            .iter()
            .map(|byte| byte.clone())
            .filter(|byte| byte[i] != most_common_bits_co2_bytes[i])
            .collect();
        // println!("Current co2 bytes: {:?}", co2_scrubber_rating_bytes);
    }
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = utils::read_input_from_path("example_input/day3_2021.txt");
    let input = utils::read_input_from_path("input/day3_2021.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    match now.elapsed() {
        Ok(elapsed) => println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
                                elapsed.as_secs(),
                                elapsed.as_millis(),
                                elapsed.as_micros()),
        Err(e) => println!("Error: {e:?}")
    }

    let now = SystemTime::now();
    println!("Part Two: {}", part_two(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    Ok(())
}