use std::time::SystemTime;
use num::traits::real::Real;
use advent_of_code_2024::utils;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct GridPoint2D {
    row: i32,
    col: i32
}

impl GridPoint2D {
    fn new(row: i32, col: i32) -> GridPoint2D {
        GridPoint2D { row, col }
    }
}

#[derive(Debug, Clone, Copy)]
struct Grid {
    top_left: GridPoint2D,
    bottom_right: GridPoint2D
}

impl Grid {
    fn new(top_left: GridPoint2D, bottom_right: GridPoint2D) -> Grid {
        Grid { top_left, bottom_right }
    }
}

#[derive(Debug, Clone, Copy)]
struct Velocity2D {
    row: i32,
    col: i32
}

impl Velocity2D {
    fn new(row: i32, col: i32) -> Velocity2D {
        Velocity2D { row, col }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: GridPoint2D,
    velocity: Velocity2D
}

impl Robot {
    fn new(position: GridPoint2D, velocity: Velocity2D) -> Robot {
        Robot { position, velocity }
    }
}

fn collect_robots(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<&str>>();
        let pos = split[0]
            .split("=")
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let position = GridPoint2D::new(pos[1], pos[0]);
        let vel = split[1]
            .split("=")
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let velocity = Velocity2D::new(vel[1], vel[0]);
        robots.push(Robot::new(position, velocity));
    }
    robots
}

fn walk_robot(robot: &Robot, grid: &Grid, amount_seconds: i32) -> Robot {
    let mut updated_robot = robot.clone();
    updated_robot.position.row = updated_robot.position.row + (robot.velocity.row * amount_seconds);
    updated_robot.position.col = updated_robot.position.col + (robot.velocity.col * amount_seconds);

    updated_robot.position.row = updated_robot.position.row % grid.bottom_right.row;
    updated_robot.position.col = updated_robot.position.col % grid.bottom_right.col;

    if updated_robot.position.row < 0 {
        updated_robot.position.row += grid.bottom_right.row;
    }
    if updated_robot.position.col < 0 {
        updated_robot.position.col += grid.bottom_right.col;
    }

    updated_robot
}

fn is_in_bounds(robot_position: &GridPoint2D, grid: &Grid) -> bool {
    /// include min max of bounds
    return robot_position.row >= grid.top_left.row
        && robot_position.row <= grid.bottom_right.row
        && robot_position.col >= grid.top_left.col
        && robot_position.col <= grid.bottom_right.col;
}

fn count_robots_in_area(robots: &Vec<Robot>, grid: &Grid) -> i32 {
    let robots_within_area: Vec<&Robot> = robots
        .iter()
        .filter(|robot| is_in_bounds(&robot.position, grid))
        .collect();

    robots_within_area.len() as i32
}

fn split_grid(grid: &Grid) -> Vec<Grid>{
    let skip_row = (grid.bottom_right.row as f32 / 2.0).floor() as i32;
    let skip_col = (grid.bottom_right.col as f32 / 2.0).floor() as i32;

    let top_left_quarter = Grid::new(
        GridPoint2D::new(0, 0),
        GridPoint2D::new(skip_row - 1, skip_col - 1)
    );
    let top_right_quarter = Grid::new(
        GridPoint2D::new(0, skip_col + 1),
        GridPoint2D::new(skip_row - 1, grid.bottom_right.col)
    );
    let bottom_left_quarter = Grid::new(
        GridPoint2D::new(skip_row + 1, grid.top_left.col),
        GridPoint2D::new(grid.bottom_right.row, skip_col - 1)
    );
    let bottom_right_quarter = Grid::new(
        GridPoint2D::new(skip_row + 1, skip_col + 1),
        GridPoint2D::new(grid.bottom_right.row, grid.bottom_right.col)
    );

    return vec![top_left_quarter, top_right_quarter, bottom_left_quarter, bottom_right_quarter];
}

fn calculate_safety_score(robots_after_observation: &Vec<Robot>, grid: &Grid) -> i32 {
    let split_grid = split_grid(grid);

    let robot_count_top_left = count_robots_in_area(robots_after_observation, &split_grid[0]);
    let robot_count_top_right = count_robots_in_area(robots_after_observation, &split_grid[1]);
    let robot_count_bottom_left = count_robots_in_area(robots_after_observation, &split_grid[2]);
    let robot_count_bottom_right = count_robots_in_area(robots_after_observation, &split_grid[3]);

    let safety_score = robot_count_top_left * robot_count_top_right * robot_count_bottom_left * robot_count_bottom_right;
    safety_score
}

fn visualize_robots(robots: &Vec<Robot>, grid: &Grid) {
    let skip_row = (grid.bottom_right.row as f32 / 2.0).floor() as i32;
    let skip_col = (grid.bottom_right.col as f32 / 2.0).floor() as i32;
    for row in 0..grid.bottom_right.row {
        if row == skip_row {
            println!();
            println!();
            continue;
        }
        for col in 0..grid.bottom_right.col {
            if col == skip_col {
                print!("   ");
                continue;
            }
            let mut robots_on_spot = 0;
            for robot in robots {
                if robot.position.col == col && robot.position.row == row {
                    robots_on_spot += 1;
                }
            }
            if robots_on_spot > 0 {
                print!(" {} ", robots_on_spot);
            }
            else {
                print!(" . ");
            }
        }
        println!();
    }
}

fn part_one(input: &str) -> i32 {
    let grid_top_left = GridPoint2D::new(0, 0);
    let grid_bottom_right = GridPoint2D::new(103, 101);
    let grid = Grid::new(grid_top_left, grid_bottom_right);
    let observe_x_seconds = 100;
    let mut robots = collect_robots(input);

    println!("Before:");
    // visualize_robots(&robots, &grid);

    for robot in &mut robots {
        *robot = walk_robot(&robot, &grid, observe_x_seconds);
        println!("Final position: {:?}", robot.position);
    }

    println!("After:");
    // visualize_robots(&robots, &grid);

    let answer = calculate_safety_score(&robots, &grid);
    answer
}

fn part_two(input: &str) -> i32 {
    0
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(14);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day14.txt");

    // 234362480 too high
    // 232444842 too high
    // 221579072 not correct
    // 88628616 not correct, maar op grid 7, 11........
    // 225552000
    let now = SystemTime::now();
    let ans = part_one(&input);
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());
    println!("Part One: {}", ans);

    let now = SystemTime::now();
    let ans_2 = part_two(&input);
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());
    println!("Part Two: {}", ans_2);

    Ok(())
}