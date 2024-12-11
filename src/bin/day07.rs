use std::time::SystemTime;
use itertools::Itertools;
use num::Num;
use advent_of_code_2024::utils;

enum Operators {
    Add,
    Sub,
    Mul,
    Div
}

fn calculate(first: i128, second: i128, operator: &Operators) -> Option<i128> {
    match operator {
        Operators::Add => Some(first + second),
        Operators::Sub => Some(first - second),
        Operators::Mul => Some(first * second),
        Operators::Div => {
            if second != 0 {
                Some(first / second)
            }
            else {
                None
            }
        }
    }
}

fn is_solvable(equation_numbers: &Vec<i128>, desired_equation_result: i128, available_operations: &Vec<Operators>) -> bool {
    let amount_equations = equation_numbers.len() - 1; // windows of 2 gives -> 2 + 3 * 4 -> [2 + 3] and [5 * 4] -> len - 1 equations
    let possible_operator_orders = (0..amount_equations)
        .map(|_| available_operations.iter())
        .multi_cartesian_product()
        .collect::<Vec<_>>();

    for operator_order in possible_operator_orders {
        let mut operator_order_result = 0;
        for (i, pair) in equation_numbers.iter().tuple_windows::<(&i128, &i128)>().enumerate() { // [2, 3, 4] -> (0, (2, 3)), (1, (3, 4)) ...
            if i == 0 {
                operator_order_result = calculate(*pair.0, *pair.1, &operator_order[i]).unwrap();
            }
            else {
                operator_order_result = calculate(operator_order_result, *pair.1, &operator_order[i]).unwrap();
            }
        }
        if operator_order_result == desired_equation_result {
            return true;
        }
    }
    false
}


fn part_one(input: &str) -> i128 {
    let available_operations: Vec<Operators> = vec![Operators::Mul, Operators::Add];
    let mut answer = 0;
    for line in input.lines() {
        let mut equation = line.split(':');
        let equation_result = equation.next().unwrap_or("geen equation result :(").parse::<i128>().unwrap();
        let equation_numbers = equation
            .next()
            .unwrap_or("geen equation :(")
            .split_whitespace()
            .filter_map(|s| s.parse::<i128>().ok())
            .collect();

        if is_solvable(&equation_numbers, equation_result, &available_operations) {
            answer += equation_result;
        }
    }
    answer
}

fn part_two(input: &str) -> i128 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(7);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day07.txt");

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