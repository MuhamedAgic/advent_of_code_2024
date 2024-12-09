use std::time::SystemTime;
use itertools::enumerate;
use advent_of_code_2024::utils;


fn get_memory_map(disk_map: &Vec<i32>) -> Vec<Option<i32>> {
    let mut memory_layout = Vec::new();
    let mut file_id = 0;
    for (i, item) in disk_map.iter().enumerate() {
        if i % 2 == 0 { // push file
            for j in 0..*item {
                memory_layout.push(Some(file_id));
                // print!("{}", file_id);
            }
            file_id += 1;
        }
        else { // push 'empty space'
            for j in 0..*item {
                memory_layout.push(None);
                // print!(".");
            }
        }
    }
    // println!("\n{:?}\n", memory_layout);
    memory_layout
}

fn print_rearranged_memory(memory: &Vec<Option<i32>>) {
    memory.iter().for_each(|item| match item {
        None => {
            print!(".");
        }
        Some(val) => {
            print!("{}", val);
        }
    });
    println!();
}

fn rearrange_memory(memory: &Vec<Option<i32>>) -> Vec<Option<i32>> {
    let mut rearranged_memory = memory.clone();
    for i in (0..memory.len()).rev() {
        match memory[i] {
            Some(value) => {
                if rearranged_memory.iter().take(i).all(|item| item.is_some()) {
                    break; // everything until now is Some(val), that means rearranged
                }
                let first_empty_spot = rearranged_memory.iter().position(|item| item.is_none()).unwrap();
                rearranged_memory[first_empty_spot] = Some(value);
                rearranged_memory[i] = None;
                // print_rearranged_memory(&rearranged_memory);
            }
            None => {}
        }
    }
    rearranged_memory
}

fn calculate_memory_checksum(memory: &Vec<Option<i32>>) -> i64 {
    memory
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| {
            match item {
                Some(value) => {
                    acc + i as i64 * *value as i64
                },
                None => acc
            }
        })
}

fn part_one(input: &str) -> i64 {
    let disk_map = input
        .chars()
        .filter_map(|c| c.to_string().parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let memory_map = get_memory_map(&disk_map);
    let rearranged_memory = rearrange_memory(&memory_map);
    let answer = calculate_memory_checksum(&rearranged_memory);
    answer
}

fn part_two(input: &str) -> i32 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(9);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day09.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    // println!("Part One V2: {}", part_two(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    Ok(())
}