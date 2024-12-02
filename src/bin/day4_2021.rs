use std::time::SystemTime;
use advent_of_code_2024::utils;

#[derive(Debug, Clone)]
struct Grid2D {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<i32>>,
}

impl Grid2D {
    fn new(rows: usize, cols: usize) -> Grid2D {
        Grid2D {
            rows,
            cols,
            grid: vec![vec![0; cols]; rows]
        }
    }
}

fn part_one(input: &String) -> i32 {
    let mut bingo_numbers_collected = false;
    let mut bingo_numbers:Vec<i32> = Vec::new();

    let mut save_bingo_card = false;
    let mut current_bingo_card_row = 0;
    let mut current_bingo_card = Grid2D::new(5, 5);
    let mut bingo_cards = Vec::new();

    for line in input.lines() {
        if !bingo_numbers_collected {
            bingo_numbers = utils::collect_numbers::<i32>(line, ',');
            bingo_numbers_collected = true;
            continue;
        }

        if line.trim().is_empty() {
            if current_bingo_card_row == 5 {
                save_bingo_card = true;
            }
            continue;
        }

        if save_bingo_card {
            let bingo_card = current_bingo_card.clone();
            bingo_cards.push(bingo_card);
            current_bingo_card.grid.clear();
            current_bingo_card_row = 0;
            save_bingo_card = false;
            continue;
        }

        let bingo_card_row = utils::collect_numbers::<i32>(&line, ' ');
        current_bingo_card.grid[current_bingo_card_row] = bingo_card_row;
        current_bingo_card_row += 1;

    }
    todo!();
}


fn main() {
    let input = utils::read_input_from_path("example_input/day4_2021.txt");
    // let input = utils::read_input_from_path("input/day4_2021.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    match now.elapsed() {
        Ok(elapsed) => println!("Seconds elapsed {} ({} milliseconds, {} microseconds)", elapsed.as_secs(), elapsed.as_millis(), elapsed.as_micros()),
        Err(e) => println!("Error: {e:?}")
    }
    // println!("Part Two: {}", part_two(&input));
}