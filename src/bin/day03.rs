use std::time::SystemTime;
use advent_of_code_2024::utils;
use regex::Regex;

fn get_mul_result(operation: &str) -> i32 {
    let mut numbers: Vec<String> = Vec::new();
    let mut number = String::from("");
    for char in operation.chars() {
        if char.is_digit(10) {
            number.push(char);
        } else {
            if !number.is_empty() {
                numbers.push(number.clone());
                number = String::from("");
            }
        }
    }
    numbers
        .iter()
        .map(|num| num.parse::<i32>().unwrap())
        .fold(1, |acc, num| acc * num) // 1 * num1 * num2
}


fn part_one(input: &str) -> i32 {
    let mut answer: i32 = 0;
    let mul_regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let operations: Vec<&str> = mul_regex
        .find_iter(input)
        .map(|found_item| found_item.as_str())
        .collect();

    for operation in operations {
        answer += get_mul_result(operation);
    }
    answer
}

fn part_two(input: &str) -> i32 {
    let mut answer: i32 = 0;
    let mul_regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let enable_regex = Regex::new(r"do\(\)").unwrap();
    let disable_regex = Regex::new(r"don't\(\)").unwrap();
    let operations: Vec<(usize, &str)> = mul_regex
        .find_iter(input)
        .map(|found_item| (found_item.start(), found_item.as_str()))
        .collect();

    let enables_locations: Vec<usize> = enable_regex
        .find_iter(input)
        .map(|found_enable| found_enable.start())
        .collect();

    let disables_locations: Vec<usize> = disable_regex
        .find_iter(input)
        .map(|found_disable| found_disable.start())
        .collect();

    let mut enabled = true; // from the start it is true
    for (i, char) in input.chars().enumerate() {
        if disables_locations.contains(&i) {
            enabled = false;
        }
        else if enables_locations.contains(&i) {
            enabled = true;
        }
        else if operations.iter().any(|(pos, operation)| *pos == i) && enabled { // if we are currently on a mul operation, and enabled
            let operation = operations
                .iter()
                .filter(|(pos, operation)| *pos == i)
                .map(|(pos, operation)| operation)
                .collect::<Vec<&&str>>();
            answer += get_mul_result(operation.get(0).unwrap());
        }
    }
    answer
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(3);
    let example_input_v1 = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day03_v1.txt");
    let example_input_v2 = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day03_v2.txt");

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