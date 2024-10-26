use itertools::Itertools;
use advent_of_code_2024::utils;

fn part_one(input: &String) -> i32 {
    let mut depth = 0;
    let mut horizontal_distance = 0;
    for line in input.lines() {
        let (direction, distance) = line.split(' ').collect_tuple().unwrap();
        let distance = distance.parse::<i32>().unwrap();
        let _ = match direction {
            "forward" => horizontal_distance += distance,
            "up" => depth -= distance,
            "down" => depth += distance,
            _ => panic!("aahh!"),
        };
    }
    println!("Horizontal: {horizontal_distance}, Depth: {depth}");
    return horizontal_distance * depth;
}

fn part_two(input: &String) -> i32 {
    let mut depth = 0;
    let mut horizontal_distance = 0;
    let mut aim = 0;
    for line in input.lines() {
        let (direction, distance) = line.split(' ').collect_tuple().unwrap();
        let distance = distance.parse::<i32>().unwrap();
        let _ = match direction {
            "forward" => {
                horizontal_distance += distance;
                depth += aim * distance;
            },
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => panic!("aahh!"),
        };
    }
    println!("Horizontal: {horizontal_distance}, Depth: {depth}");
    return horizontal_distance * depth;
}

fn main() {
    // let input = utils::read_input_from_path("example_input/day2_2021.txt");
    let input = utils::read_input_from_path("input/day2_2021.txt");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}