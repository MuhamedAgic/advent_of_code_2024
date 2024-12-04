use std::time::SystemTime;
use advent_of_code_2024::utils;
use rfd::FileDialog;

fn check_target_north(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row < target.len() - 1 {
        return false; // can't have XMAS, need minimum space of 4 chars
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row - i][col]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_south(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row > grid.len() - target.len() {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row + i][col]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_east(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if col > grid[row].len() - target.len() {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row][col + i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_west(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if col < target.len() - 1{
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row][col - i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_north_east(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row < target.len() - 1 || col > grid[row].len() - target.len() {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row - i][col + i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_north_west(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row < target.len() - 1 || col < target.len() - 1 {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row - i][col - i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_south_east(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row > grid.len() - target.len() || col > grid[row].len() - target.len() {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row + i][col + i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_south_west(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row > grid[row].len() - target.len() || col < target.len() - 1 {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row + i][col - i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}


fn get_targets_at(row: usize,
                  col: usize,
                  grid: &Vec<Vec<char>>,
                  target: &str,
                  search_functions: &Vec<fn(row: usize, col: usize, grid: &Vec<Vec<char>>, to_find: &str) -> bool>) -> i32 {
    if grid[row][col] != 'X' {
        return 0;
    }

    // apply every search function on location (row, col)
    search_functions
        .iter()
        .map(|function| function(row, col, &grid, &target))   // execute search function
        .filter(|search_result| *search_result)                    // keep only instances where search function has found
        .count() as i32                                                   // count how many functions have found something
}

fn part_one(input: &str) -> i32 {
    let search_functions = vec![check_target_north, check_target_south, check_target_east, check_target_west,
                                               check_target_north_east, check_target_north_west, check_target_south_east, check_target_south_west];
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut found_xmases = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            let found = get_targets_at(i, j, &grid, "XMAS", &search_functions);
            found_xmases += found;
        }
    }
    found_xmases
}

fn part_two(input: &str) -> i32 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(4);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day04.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    // println!("Part two: {}", part_two(&example_input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    Ok(())
}