
pub mod utils {
    use std::fs;
    use std::path::Path;

    pub fn read_input(day: u8) -> String {
        let input_path = format!("input/day{:02}.txt", day);
        let input = fs::read_to_string(Path::new(&input_path)).expect("Failed to read input file");
        input
    }

}
