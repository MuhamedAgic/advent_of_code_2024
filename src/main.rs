use std::time::Instant;

macro_rules! run_day {
    ($day:expr) => {{
        print!("Day {:02}: ", $day);
        let start = Instant::now();
        let result = std::process::Command::new(format!("target/release/day{:02}", $day))
            .output()
            .expect("Failed to execute command");
        let duration = start.elapsed();
        println!("{}", String::from_utf8_lossy(&result.stdout));
        println!("Time: {:?}", duration);
    }};
}

fn main() {
    run_day!(01);
    run_day!(02);
    // Add more days as you progress
}
