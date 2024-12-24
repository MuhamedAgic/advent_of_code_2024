use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufRead;
use std::time::SystemTime;
use itertools::Itertools;
use advent_of_code_2024::utils;


fn get_connected_pcs<'a>(pc: &'a str, pcs: &Vec<(&'a str, &'a str)>) -> Vec<&'a str> {
    let mut connected_pcs = Vec::new();
    for p in pcs {
        if p.0 == pc {
            connected_pcs.push(p.1);
        }
        else if p.1 == pc {
            connected_pcs.push(p.0);
        }
    }
    return connected_pcs;
}

fn part_one(input: &str) -> i32 {
    let pcs_connections = input
        .lines()
        .map(|line| line.split('-').collect_tuple::<(&str, &str)>().unwrap())
        .collect::<Vec<(&str, &str)>>();
    println!("{:?}\n", pcs_connections.len());

    let unique_pcs: Vec<&str> = pcs_connections
        .iter()
        .flat_map(|(first, second)| vec![first.clone(), second.clone()])
        .unique()
        .collect();
    println!("length: {}", unique_pcs.len());

    let mut triangle_connections = Vec::new();
    for pc in unique_pcs {
        // println!("Checking connections for pc: {:?}", pc);
        let connected_pcs = get_connected_pcs(pc, &pcs_connections);
        // println!("Connected with pc: {:?} -> {:?}", pc, connected_pcs);
        for &connected_pc in connected_pcs.iter() { // for every connection, check their connection as well
            let connected_pc_connections = get_connected_pcs(connected_pc, &pcs_connections);
            // println!("pc connection: {:?} connects with -> {:?}", connected_pc, connected_pc_connections);
            for connected_pc_connection in connected_pc_connections {
                if connected_pcs.contains(&connected_pc_connection) { // got triangle here
                    // println!("Got three: {},{},{}", pc, connected_pc, connected_pc_connection);
                    if triangle_connections
                        .iter()
                        .any(|(pc1, pc2, pc3): &(&str, &str, &str)| {
                            (pc1.eq(&pc) || pc2.eq(&pc) || pc3.eq(&pc))
                            && (pc1.eq(&connected_pc) || pc2.eq(&connected_pc) || pc3.eq(&connected_pc))
                            && (pc1.eq(&connected_pc_connection) || pc2.eq(&connected_pc_connection) || pc3.eq(&connected_pc_connection))
                        }) {
                        // println!("Exists already in another form!");
                        continue;
                    }
                    else {
                        triangle_connections.push((pc, connected_pc, connected_pc_connection));
                    }
                }
            }
        }
        // println!();
    }

    triangle_connections = triangle_connections.iter().unique().cloned().collect();
    // println!("\n\n\nFinal three: \n{:?}\n\n\n", triangle_connections);

    let final_pcs: Vec<_> = triangle_connections
        .iter()
        .filter(|(pc, connected_pc, connected_pc_connection)| {
            pc.contains('t') || connected_pc.contains('t') || connected_pc_connection.contains('t')
        })
        .collect();

    // println!("\n\n\nFinal pcs which start with t: \n{:?}\n\n\n", final_pcs);
    final_pcs.iter().count() as i32
}

fn part_two(input: &str) -> i32 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(23);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day23.txt");

    // 2331 too high
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