
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
        let numbers = line
            .trim()
            .split(separator)
            .map(str::trim)
            .filter_map(|x| x.parse::<T>().ok())
            .collect::<Vec<T>>();
        numbers
    }

    fn abs(val: i32) -> i32 {
        if val < 0 {
            return -val;
        }
        val
    }


    pub mod bit {
        const BIT_MATRIX_IDENTITY_TRUE: [u8; 8] = [
            0b10000000,
            0b01000000,
            0b00100000,
            0b00010000,
            0b00001000,
            0b00000100,
            0b00000010,
            0b00000001
        ];

        const BIT_MATRIX_IDENTITY_FALSE: [u8; 8] = [
            0b01111111,
            0b10111111,
            0b11011111,
            0b11101111,
            0b11110111,
            0b11111011,
            0b11111101,
            0b11111110
        ];

        fn set_bit_true(mut byte: u8, pos: u8) -> Option<u8> {
            match pos {
                0 => byte |= BIT_MATRIX_IDENTITY_TRUE[0],
                1 => byte |= BIT_MATRIX_IDENTITY_TRUE[1],
                2 => byte |= BIT_MATRIX_IDENTITY_TRUE[2],
                3 => byte |= BIT_MATRIX_IDENTITY_TRUE[3],
                4 => byte |= BIT_MATRIX_IDENTITY_TRUE[4],
                5 => byte |= BIT_MATRIX_IDENTITY_TRUE[5],
                6 => byte |= BIT_MATRIX_IDENTITY_TRUE[6],
                7 => byte |= BIT_MATRIX_IDENTITY_TRUE[7],
                _ => return None,
            };
            Some(byte)
        }

        fn set_bit_false(mut byte: u8, pos: u8) -> Option<u8> {
            match pos {
                0 => byte &= BIT_MATRIX_IDENTITY_FALSE[0],
                1 => byte &= BIT_MATRIX_IDENTITY_FALSE[1],
                2 => byte &= BIT_MATRIX_IDENTITY_FALSE[2],
                3 => byte &= BIT_MATRIX_IDENTITY_FALSE[3],
                4 => byte &= BIT_MATRIX_IDENTITY_FALSE[4],
                5 => byte &= BIT_MATRIX_IDENTITY_FALSE[5],
                6 => byte &= BIT_MATRIX_IDENTITY_FALSE[6],
                7 => byte &= BIT_MATRIX_IDENTITY_FALSE[7],
                _ => return None,
            };
            Some(byte)
        }

        fn set_bit(byte: u8, val: u8, pos: u8) -> Option<u8> {
            match val {
                0 => {
                    if let (byte) = set_bit_false(byte, pos) {
                        return byte;
                    }
                    None
                }
                1 => {
                    if let (byte) = set_bit_true(byte, pos) {
                        return byte;
                    }
                    None
                }
                _ => None
            }
        }

        fn get_bits(byte: u8) -> [u8; 8] {
            let mut bits = [0u8; 8];
            for i in 0..8 {
                let shifted_byte = byte >> i; // Get the rightmost bit of the shifted byte (least significant bit)
                let cur_bit = shifted_byte & 1;
                // For the first iteration, the cur_bit is the
                // least significant bit and therefore we place
                // that bit at index 7 of the array (rightmost bit)
                bits[7 - i] = cur_bit;
            }
            bits
        }
    }

}
