use std::time::SystemTime;
use advent_of_code_2024::utils;


fn calculate_next_secret_num(num: i64) -> i64 {
    // mixing: xor with num * 64, pruning, modulo 16777216
    let step_one = (num ^ (num * 64)) % 16777216;
    let step_two = ((step_one as f64 / 32.0).floor() as i64 ^ step_one) % 16777216;
    let step_three = ((step_two * 2048) ^ step_two) % 16777216;
    step_three
}


fn part_one(input: &str) -> i64 {
    let nums = input
        .lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let after_2000: Vec<i64> = nums
        .iter()
        .map(|num| {
            let mut num_after_2000 = num.clone();
            for i in 0..2000 {
                num_after_2000 = calculate_next_secret_num(num_after_2000);
            }
            return num_after_2000;
        })
        .collect();

    after_2000.iter().sum()
}

fn part_two(input: &str) -> i32 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(22);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day22.txt");

    // 4973560 too low
    let now = SystemTime::now();
    let ans = part_one(&input);
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());
    println!("Part One: {}", ans);

    // let now = SystemTime::now();
    // let ans_2 = part_two(&input);
    // println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
    //          now.elapsed()?.as_secs(),
    //          now.elapsed()?.as_millis(),
    //          now.elapsed()?.as_micros());
    // println!("Part Two: {}", ans_2);

    Ok(())
}