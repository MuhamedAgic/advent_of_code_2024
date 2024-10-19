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
    let position_coordinates = utils::collect_numbers_from(position_and_velocity.get(0).unwrap(), ',');
    let velocity_speeds = utils::collect_numbers_from(position_and_velocity.get(1).unwrap(), ',');

    if position_coordinates.len() != 3 || velocity_speeds.len() != 3 {
        return None;
    }

    let position = Position::new(position_coordinates[0], position_coordinates[1], position_coordinates[2]);
    let velocity = Velocity::new(velocity_speeds[0], velocity_speeds[1], velocity_speeds[2]);
    let hail_stone = HailStone { position, velocity };
    Some(hail_stone)
}

fn calculate_hailstones_intersection_point(hail_stone_a: &HailStone, hail_stone_b: &HailStone) -> Option<Position> {
    // ax + b = cx + d, or ax + b = cx - d, or ax - b = cx - d, or ax - b = cx + d
    let rico_hail_stone_a = hail_stone_a.velocity.y as f64 / hail_stone_a.velocity.x as f64;
    let ax_hailstone_a = hail_stone_a.position.x as f64 * rico_hail_stone_a; // get ax from y = ax + b
    let mut intersection_hailstone_a_y_axis =  hail_stone_a.position.y - ax_hailstone_a;     // calculate b

    let rico_hail_stone_b = hail_stone_b.velocity.y as f64 / hail_stone_b.velocity.x as f64;
    let ax_hailstone_b = hail_stone_b.position.x as f64 * rico_hail_stone_b; // get ax from y = ax + b
    let mut intersection_hailstone_b_y_axis = hail_stone_b.position.y - ax_hailstone_b; // calculate b

    if rico_hail_stone_a == rico_hail_stone_b {
        return None // parallel
    }

    let mut lhs_equation = 0.0; // 2x = ...
    if rico_hail_stone_b.is_sign_negative() {
        lhs_equation = rico_hail_stone_a + rico_hail_stone_b;
    }
    else {
        lhs_equation = rico_hail_stone_a - rico_hail_stone_b;
    }

    let mut rhs_equation = 0.0; // e.g. ... = 5
    if intersection_hailstone_a_y_axis.is_sign_negative() {
        rhs_equation = intersection_hailstone_b_y_axis + intersection_hailstone_a_y_axis;
    }
    else {
        rhs_equation = intersection_hailstone_b_y_axis - intersection_hailstone_a_y_axis;
    }

    // e.g. 2x = 5 -> x = 5/2 = 2.5
    let intersection_point_2d_x = rhs_equation / lhs_equation;
    let intersection_point_2d_y = (rico_hail_stone_a * intersection_point_2d_x) + intersection_hailstone_a_y_axis; // y = ax + b

    let intersection_point_2d = Position::new(intersection_point_2d_x, intersection_point_2d_y, 0.0);
    return Some(intersection_point_2d);
}

fn hailstone_crosses_area(hail_stone: &HailStone, area: &Area) -> bool {
    let rico_hail_stone = hail_stone.velocity.y as f64 / hail_stone.velocity.x as f64;
    let intersection_point_y_axis_hailstone = hail_stone.position.x as f64 * -rico_hail_stone;

    if rico_hail_stone == 0.0
        && intersection_point_y_axis_hailstone < area.min_y
        && intersection_point_y_axis_hailstone > area.max_y {
        return false; // horizontal line which does not cross the area, we don't assume there are vertical lines
    }

    // calculate y value for line at area min x
    let y_val_at_area_min_x = rico_hail_stone * area.min_x + intersection_point_y_axis_hailstone; // y = ax + b
    if y_val_at_area_min_x < area.max_y && y_val_at_area_min_x > area.min_y {
        return true;
    }

    //calculate x value for intersection point with roof and floor of the area
    let x_val_at_area_max_y = (area.max_y - intersection_point_y_axis_hailstone) / rico_hail_stone; // ax = y - b -> x = (y - b) / a
    if x_val_at_area_max_y > area.min_x && x_val_at_area_max_y < area.max_x {
        return true; // intersection is within x boundaries for roof of area
    }

    let x_val_at_area_min_y = (area.min_y - intersection_point_y_axis_hailstone) / rico_hail_stone; // ax = y - b -> x = (y - b) / a
    if x_val_at_area_min_y > area.min_x && x_val_at_area_min_y < area.max_x {
        return true; // intersection is within x boundaries for floor of area
    }

    return false;
}

fn is_point_within_area(point: &Position, area: &Area) -> bool {
    return point.x < area.max_x && point.x > area.min_x  && point.y < area.max_y && point.y > area.min_y;
}

fn count_hailstone_collisions(hail_stones: &Vec<HailStone>, test_area: &Area) -> i32 {
    let mut count = 0;
    for hail_stone_pair in hail_stones.iter().combinations(2) {
        let intersection_point = calculate_hailstones_intersection_point(hail_stone_pair[0], hail_stone_pair[1]);
        // println!("Comparing hailstones: \n    {:?}\n    {:?}\n", hail_stone_pair[0], hail_stone_pair[1]);
        match intersection_point {
            Some(position) => {
                // println!("Got intersection point {:?}\n", &position);
                if is_point_within_area(&position, &test_area) {
                    // println!("Intersection point {:?} is within area {:?}\n", &position, &test_area);
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

fn count_hailstone_pairs_crossing_area(hail_stones: &Vec<HailStone>, test_area: &Area) -> i32 {
    let mut count = 0;
    for hail_stone_pair in hail_stones.iter().combinations(2) {
        let hail_stone_a_intersects_area = hailstone_crosses_area(hail_stone_pair[0], &test_area);
        let hail_stone_b_intersects_area = hailstone_crosses_area(hail_stone_pair[1], &test_area);
        if hail_stone_a_intersects_area && hail_stone_b_intersects_area {
            count += 1;
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
    let answer = count_hailstone_collisions(&hailstones, &test_area);
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