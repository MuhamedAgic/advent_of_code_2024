use std::time::SystemTime;
use num::traits::real::Real;
use regex::Regex;
use advent_of_code_2024::utils;

#[derive(Debug, Copy, Clone)]
struct Coordinate2D {
    x: i64,
    y: i64
}

impl Coordinate2D {
    fn new() -> Coordinate2D {
        Coordinate2D { x: 0, y: 0 }
    }

    fn from(x: i64, y: i64) -> Coordinate2D {
        Coordinate2D { x, y }
    }

}

#[derive(Debug, Copy, Clone)]
struct Button {
    dx: i64,
    dy: i64
}

impl Button {
    fn new() -> Button {
        Button { dx: 0, dy: 0 }
    }

    fn from(x: i64, y: i64) -> Button {
        Button { dx: x, dy: y }
    }
}

#[derive(Debug, Copy, Clone)]
struct ClawMachine {
    a: Button,
    b: Button,
    prize: Coordinate2D
}

impl ClawMachine {
    fn new() -> ClawMachine {
        ClawMachine {
            a: Button::new(), b: Button::new(), prize: Coordinate2D { x: 0, y: 0 }
        }
    }
}

fn collect_numbers_from_line(line: &str) -> Option<Vec<i64>> {
    let regex = Regex::new("\\d+").unwrap();
    let numbers = regex
        .find_iter(line)
        .map(|mat| mat.as_str().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    if numbers.is_empty() {
        return None;
    }
    Some(numbers)
}

fn collect_claw_machines(input: &str, target_x_offset: i64, target_y_offset: i64) -> Vec<ClawMachine> {
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let mut claw_machine = ClawMachine::new();
    let mut part_count = 0;
    for line in input.lines() {
        if line.is_empty() {
            part_count = 0;
            continue;
        }
        if let Some(numbers) = collect_numbers_from_line(&line) {
            if part_count == 0 { // collect button a
                claw_machine.a = Button::from(numbers[0], numbers[1]);
            }
            else if part_count == 1 { // collect button b
                claw_machine.b = Button::from(numbers[0], numbers[1]);
            }
            else if part_count == 2 { // collect prize
                claw_machine.prize = Coordinate2D::from(numbers[0] + target_x_offset, numbers[1] + target_y_offset);
                claw_machines.push(claw_machine.clone());
            }
            else {
                println!("Kan niet! {}", line);
            }
        }
        part_count += 1;
    }
    claw_machines
}

fn meets_criteria_part_one(button_a_count: f64, button_b_count: f64, push_limit: i64) -> bool {
    return (button_b_count >= 0.0 && button_b_count <= push_limit as f64)
        && (button_a_count >= 0.0 && button_a_count <= push_limit as f64)
        && (button_a_count.fract() == 0.0 && button_b_count.fract() == 0.0) // it was the "exactly" keyword that went wrong...
}

fn meets_criteria_part_two(button_a_count: f64, button_b_count: f64, push_limit: i64) -> bool {
    return (button_b_count >= 0.0 && button_a_count >= 0.0)
        && (button_a_count.fract() == 0.0 && button_b_count.fract() == 0.0) // it was the "exactly" keyword that went wrong...
}

fn get_prize_within_reach(claw_machine: &ClawMachine, push_limit: i64, meets_criteria: fn(f64, f64, i64) -> bool) -> Option<(i64, i64)> {
    // 94a + 22b = 8400 -> button_a_dx * unknown + button_b_dx * unknown = 8400
    // 34a + 67b = 5400 -> button_a_dy * unknown + button_b_dy * unknown = 5400

    // calculate a, mul first equation
    let new_dx_button_b = claw_machine.b.dx * claw_machine.a.dy;
    let new_prize_x = claw_machine.prize.x * claw_machine.a.dy;

    // mul second equation
    let new_dy_button_b = claw_machine.b.dy * claw_machine.a.dx;
    let new_prize_y = claw_machine.prize.y * claw_machine.a.dx;

    // situation now
    // 3196a + 748b = 285600
    // 3196a + 6298b = 507600
    let d_b = new_dx_button_b - new_dy_button_b;
    let d_target = new_prize_x - new_prize_y;
    let button_b_count = (d_target as f64 / d_b as f64);

    // 94a + 22 * button_b_count = 8400
    // a = (8400 - (22 * button_b_count)) / 94
    let button_a_count = ((claw_machine.prize.x as f64 - (claw_machine.b.dx as f64 * button_b_count)) / claw_machine.a.dx as f64);

    if meets_criteria(button_a_count, button_b_count, push_limit) { // it was the "exactly" keyword that went wrong...
        return Some((button_a_count as i64, button_b_count as i64));
    }
    None
}

fn part_one_and_two(input: &str, x_offset_part_x: i64, y_offset_part_x: i64) -> i64 {
    let claw_machines = collect_claw_machines(input, x_offset_part_x, y_offset_part_x);
    let mut total_used_coins = 0;
    let mut criteria_fn: fn(f64, f64, i64) -> bool = meets_criteria_part_one;
    if x_offset_part_x > 0 { // part 2
        criteria_fn = meets_criteria_part_two;
    }
    for claw_machine in claw_machines {
        if let Some((a_button_push_count, b_button_push_count)) = get_prize_within_reach(&claw_machine, 100, criteria_fn) {
            total_used_coins += (a_button_push_count * 3) + b_button_push_count;
        }
    }
    total_used_coins
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(13);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day13.txt");

    // 32297 too high
    // 32455 too high
    // 32607, must be too high as well...
    // 34157 too high

    let (x_offset_part_one, y_offset_part_one) = (0, 0);
    let (x_offset_part_two, y_offset_part_two) = (10000000000000, 10000000000000);

    let now = SystemTime::now();
    let ans = part_one_and_two(&input, x_offset_part_one, y_offset_part_one);
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());
    println!("Part One: {}\n", ans);


    let now = SystemTime::now();
    let ans = part_one_and_two(&input, x_offset_part_two, y_offset_part_two);
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());
    println!("Part Two: {}", ans);

    Ok(())
}