use std::time::Instant;

macro_rules! run_day {
    ($day:expr) => {{
        print!("Day {:02}: \n", $day);
        let start = Instant::now();
        let exe_name = format!("day{:02}.exe", $day);
        let exe_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(&exe_name);
        let result = std::process::Command::new(exe_path)
            .output()
            .expect(&format!("Failed to execute {}", exe_name));
        let duration = start.elapsed();
        println!("{}", String::from_utf8_lossy(&result.stdout));
        println!("Time: {:?}", duration);
    }};
}

fn main() {
    run_day!(01);
    // run_day!(02);
    // Add more days as you progress
}
