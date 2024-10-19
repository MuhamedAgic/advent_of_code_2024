use itertools::Itertools;
use advent_of_code_2024::utils;

#[derive(Debug, Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
    z: f64
}

impl Position {
    fn new(x: f64, y: f64, z: f64) -> Position {
        Position { x, y, z }
    }
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    /// Velocity in xyz axis
    x: f64,
    y: f64,
    z: f64
}

impl Velocity {
    fn new(x: f64, y: f64, z: f64) -> Velocity {
        Velocity { x, y, z }
    }
}

#[derive(Debug, Copy, Clone)]
struct HailStone {
    position: Position,
    velocity: Velocity
}

#[derive(Debug, Copy, Clone)]
struct Area {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64
}

impl Area {
    fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Area {
        Area { min_x, min_y, max_x, max_y }
    }
}

fn get_hailstone_from_line(line: &str) -> Option<HailStone> {
    let position_and_velocity = line.split("@").collect::<Vec<&str>>();
    let position_coordinates = utils::collect_numbers::<f64>(position_and_velocity.get(0).unwrap(), ',');
    let velocity_speeds = utils::collect_numbers::<f64>(position_and_velocity.get(1).unwrap(), ',');

    if position_coordinates.len() != 3 || velocity_speeds.len() != 3 {
        return None;
    }

    let position = Position::new(position_coordinates[0], position_coordinates[1], position_coordinates[2]);
    let velocity = Velocity::new(velocity_speeds[0], velocity_speeds[1], velocity_speeds[2]);
    let hail_stone = HailStone { position, velocity };
    Some(hail_stone)
}

fn derive_line_equation(hail_stone: &HailStone) -> (f64, f64) {
    /// gets the a and b from y = ax + b
    let a_hail_stone = hail_stone.velocity.y / hail_stone.velocity.x; // slope (a)
    let ax_hailstone = hail_stone.position.x * a_hail_stone; // get ax from y = ax + b
    let b_hail_stone =  hail_stone.position.y - ax_hailstone;  // calculate b
    return (a_hail_stone, b_hail_stone);
}

fn calculate_hailstones_intersection_point(hail_stone_a: &HailStone, hail_stone_b: &HailStone) -> Option<Position> {
    // ax + b = cx + d, or ax + b = cx - d, or ax - b = cx - d, or ax - b = cx + d
    let (a_hail_stone_a, b_hail_stone_a) = derive_line_equation(hail_stone_a);
    let (a_hail_stone_b, b_hail_stone_b) = derive_line_equation(hail_stone_b);

    println!("A: y = {}x + {}", a_hail_stone_a, b_hail_stone_a);
    println!("B: y = {}x + {}", a_hail_stone_b, b_hail_stone_b);

    if a_hail_stone_a == a_hail_stone_b {
        return None // parallel
    }

    let mut lhs_equation = 0.0; // 2x = ...
    if a_hail_stone_b.is_sign_negative() {
        lhs_equation = a_hail_stone_a + a_hail_stone_b;
    }
    else {
        lhs_equation = a_hail_stone_a - a_hail_stone_b;
    }

    let mut rhs_equation = 0.0; // e.g. ... = 5
    if b_hail_stone_a.is_sign_negative() {
        rhs_equation = b_hail_stone_b + b_hail_stone_a;
    }
    else {
        rhs_equation = b_hail_stone_b - b_hail_stone_a;
    }

    // e.g. 2x = 5 -> x = 5/2 = 2.5
    let intersection_point_2d_x = rhs_equation / lhs_equation;
    let intersection_point_2d_y = (a_hail_stone_a * intersection_point_2d_x) + b_hail_stone_a; // y = ax + b

    let intersection_point_2d = Position::new(intersection_point_2d_x, intersection_point_2d_y, 0.0);
    return Some(intersection_point_2d);
}

fn is_point_within_area(point: &Position, area: &Area) -> bool {
    return point.x < area.max_x && point.x > area.min_x  && point.y < area.max_y && point.y > area.min_y;
}

fn hail_stone_has_not_reached_path_intersection(hail_stone: &HailStone,
                                                intersection_point: &Position) -> bool {
    let from_right_to_left = hail_stone.velocity.x.is_sign_negative();
    let from_roof_to_floor = hail_stone.velocity.y.is_sign_negative();
    if from_right_to_left {
        if from_roof_to_floor {
                // x and y value of hailstone must be higher than intersection point
                return hail_stone.position.y > intersection_point.y
                && hail_stone.position.x > intersection_point.x;
        }
        else {
            // y value of hailstone must be lower, x value must be higher than intersection point
            return hail_stone.position.y < intersection_point.y
                && hail_stone.position.x > intersection_point.x;
        }
    }
    else {
        if from_roof_to_floor {
            // y value must be higher, x lower
            return hail_stone.position.y > intersection_point.y
                && hail_stone.position.x < intersection_point.x;
        }
        else {
            // x and y must be lower than intersection point xy
            return hail_stone.position.y < intersection_point.y
                && hail_stone.position.x < intersection_point.x;
        }
    }
}

fn count_hailstone_path_crossings_within_area(hail_stones: &Vec<HailStone>, test_area: &Area) -> i32 {
    let mut count = 0;
    for hail_stone_pair in hail_stones.iter().combinations(2) {
        let intersection_point = calculate_hailstones_intersection_point(hail_stone_pair[0], hail_stone_pair[1]);
        println!("Comparing hailstones: \n    {:?}\n    {:?}\n", hail_stone_pair[0], hail_stone_pair[1]);
        match intersection_point {
            Some(position) => {
                // println!("Got intersection point {:?}\n", &position);
                if is_point_within_area(&position, &test_area)
                && hail_stone_has_not_reached_path_intersection(&hail_stone_pair[0], &position)
                && hail_stone_has_not_reached_path_intersection(&hail_stone_pair[1], &position) {
                    println!("Future intersection point {:?} is within area {:?}\n", &position, &test_area);
                    count += 1;
                }
            }
            None => {
                // println!("Intersection point not found\n");
            }
        }
    }
    return count;
}

fn part_one(input: &str) -> i32 {
    let mut hailstones: Vec<HailStone> = Vec::new();
    for line in input.lines() {
        let hailstone = get_hailstone_from_line(line);
        match hailstone {
            Some(hailstone) => hailstones.push(hailstone),
            None => ()
        }
    }

    // let test_area = Area::new(7.0, 7.0, 27.0, 27.0);
    let test_area = Area::new(200000000000000.0, 200000000000000.0, 400000000000000.0, 400000000000000.0);
    let answer = count_hailstone_path_crossings_within_area(&hailstones, &test_area);
    answer
}

fn part_two(input: &str) -> i32 {
    0
}

fn main() {
    // let input = utils::read_input_from_path("example_input/day24_2023.txt");
    let input = utils::read_input_from_path("input/day24_2023.txt");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}