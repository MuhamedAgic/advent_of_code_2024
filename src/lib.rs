
pub mod utils {
    use std::fs;
    use std::path::Path;

    pub fn read_input(day: u8) -> String {
        let input_path = format!("input/day{:02}.txt", day);
        let input = fs::read_to_string(Path::new(&input_path)).expect("Failed to read input file");
        input
    }

    pub fn read_input_from_path(input_path: &str) -> String {
        let input = fs::read_to_string(Path::new(&input_path)).expect("Failed to read input file");
        input
    }

    pub fn collect_numbers_from(line: &str, separator: char) -> Vec<f64> { // todo voor verschillende typen getallen...
        let numbers = line.split(separator)
            .map(str::trim)
            .map(|x| x.parse::<f64>().unwrap()).collect();
        numbers
    }


}
