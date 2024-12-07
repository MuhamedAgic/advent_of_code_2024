use std::time::SystemTime;
use itertools::Itertools;
use advent_of_code_2024::utils;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct GridPoint2D {
    row: i32,
    col: i32
}

impl GridPoint2D {
    fn new(x: i32, y: i32) -> GridPoint2D {
        GridPoint2D { row: x, col: y }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

fn find_mark_coordinate(grid: &Vec<&str>, mark: char) -> Option<GridPoint2D> {
    /// Returns only the first found mark
    for (i, line) in grid.iter().enumerate() { // row
        for (j, str) in line.chars().enumerate() { // col
            if str == mark {
                let found_mark_coordinate = GridPoint2D::new(i as i32, j as i32);
                return Some(found_mark_coordinate);
            }
        }
    }
    None
}

fn is_within_bounds(grid: &Vec<&str>, point: &GridPoint2D) -> bool {
    return (point.row >= 0 && point.row < grid.len() as i32)
        && (point.col >= 0 && point.col < grid[0].len() as i32);
}

fn take_step(curr_pos: &GridPoint2D, direction: &Direction) -> GridPoint2D {
    match direction {
        Direction::North => return GridPoint2D::new(curr_pos.row - 1, curr_pos.col),
        Direction::South => return GridPoint2D::new(curr_pos.row + 1, curr_pos.col),
        Direction::East => return GridPoint2D::new(curr_pos.row, curr_pos.col + 1),
        Direction::West => return GridPoint2D::new(curr_pos.row, curr_pos.col - 1),
        Direction::NorthEast => return GridPoint2D::new(curr_pos.row - 1, curr_pos.col + 1),
        Direction::NorthWest => return GridPoint2D::new(curr_pos.row - 1, curr_pos.col - 1),
        Direction::SouthEast => return GridPoint2D::new(curr_pos.row + 1, curr_pos.col + 1),
        Direction::SouthWest => return GridPoint2D::new(curr_pos.row + 1, curr_pos.col - 1)
    }
}


fn get_coordinates_until(grid: &Vec<&str>, curr_pos: &GridPoint2D, direction: &Direction, stop_sign: char) -> Vec<GridPoint2D> {
    let mut walked_coordinates: Vec<GridPoint2D> = Vec::new();
    let mut pos = curr_pos.clone();
    walked_coordinates.push(pos.clone()); // or not?
    while grid[pos.row as usize].chars().nth(pos.col as usize) != Some(stop_sign) {
            pos = take_step(&pos, &direction);
            walked_coordinates.push(pos.clone());
            if !is_within_bounds(&grid, &pos) {
                break; // crashes if next while iteration is out of bounds
            }
    }
    walked_coordinates.remove(walked_coordinates.len() - 1); // remove coordinate on stop sign
    return walked_coordinates;
}

fn rotate_90_clockwise(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
        Direction::NorthEast => Direction::SouthEast,
        Direction::NorthWest => Direction::NorthEast,
        Direction::SouthEast => Direction::SouthWest,
        Direction::SouthWest => Direction::NorthWest
    }
}

fn will_go_out_of_bounds(grid: &Vec<&str>, curr_pos: &GridPoint2D, direction: &Direction) -> bool {
    let mut pos = curr_pos.clone();
    pos = take_step(&pos, &direction);
    if !is_within_bounds(&grid, &pos) {
        return true;
    }
    return false;
}

fn part_one(input: &str) -> i32 {
    let grid: Vec<&str> = input.lines().collect();

    let stop_sign = '#';
    let mut curr_direction = Direction::North;
    let mut curr_pos = find_mark_coordinate(&grid, '^').unwrap();
    let mut total_walked_coordinates: Vec<Vec<GridPoint2D>> = Vec::new();

    while is_within_bounds(&grid, &curr_pos) {
        let walked_coordinates_in_direction = get_coordinates_until(&grid, &curr_pos, &curr_direction, stop_sign);
        total_walked_coordinates.push(walked_coordinates_in_direction.clone());
        curr_pos = *walked_coordinates_in_direction.get(walked_coordinates_in_direction.len() - 1).unwrap();
        if will_go_out_of_bounds(&grid, &curr_pos, &curr_direction) {
            break;
        }
        curr_direction = rotate_90_clockwise(&curr_direction);
    }

    // walked out of grid, start counting distinct coordinates
    total_walked_coordinates
        .iter()
        .flatten()
        .unique()
        .count() as i32
}

fn part_two(input: &str) -> i32 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(6);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day06.txt");

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