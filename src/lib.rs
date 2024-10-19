
pub mod utils {
    use std::fmt::Debug;
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;

    pub fn read_input(day: u8) -> String {
        let input_path = format!("input/day{:02}.txt", day);
        let input = fs::read_to_string(Path::new(&input_path)).expect("Failed to read input file");
        input
    }

    pub fn read_input_from_path(input_path: &str) -> String {
        let input = fs::read_to_string(Path::new(&input_path)).expect("Failed to read input file");
        input
    }

    // collect numbers works with any numeric type that implements FromStr
    pub fn collect_numbers<T>(line: &str, separator: char) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug {
        let numbers = line.split(separator)
            .map(str::trim)
            .map(|x| x.parse::<T>().unwrap())
            .collect::<Vec<T>>();
        numbers
    }

}
