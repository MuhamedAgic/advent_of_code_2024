use std::time::SystemTime;
use advent_of_code_2024::utils;

fn adv (combo_op: i32, reg_a: i32) -> i32 {
    // The adv instruction (opcode 0) performs division.
    // The numerator is the value in the A register.
    // The denominator is found by raising 2 to the power of the instruction's combo operand.
    // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    // The result of the division operation is truncated to an integer and then written to the A register.

    reg_a / 2_i32.pow(combo_op as u32)
}

fn bxl (literal_op: i32, reg_b: i32) -> i32 {
    // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
    // then stores the result in register B.

    return literal_op ^ reg_b
}

fn bst (combo_op: i32) -> i32 {
    // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
    // (thereby keeping only its lowest 3 bits), then writes that value to the B register.

    combo_op % 8
}

fn jnz (literal_op: i32, reg_a: i32, instruction_ptr: usize) -> usize {
    // The jnz instruction (opcode 3) does nothing if the A register is 0.
    // However, if the A register is not zero,
    // it jumps by setting the instruction pointer to the value of its literal operand;
    // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.

    if reg_a != 0 {
        return literal_op as usize;
    }
    return instruction_ptr + 2;
}

fn bxc (reg_b: i32, reg_c: i32) -> i32 {
    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
    // then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    reg_b ^ reg_c
}

fn out (combo_op: i32) -> i32 {
    // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
    // then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    combo_op % 8
}


fn interpret_combo_operand(operand: i32, reg_a: i32, reg_b: i32, reg_c: i32) -> i32 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        7 => -999,
        _ => -9999
    }
}

fn perform_instruction(opcode: i32,
                       literal_operand: i32,
                       combo_operand: i32,
                       instruction_ptr: &mut usize,
                       reg_a: &mut i32,
                       reg_b: &mut i32,
                       reg_c: &mut i32) -> Option<i32> {
    if opcode != 3{
        *instruction_ptr += 2;
    }
    match opcode {
        0 => {
            *reg_a = adv(combo_operand, *reg_a);
            None
        },
        1 => {
            *reg_b = bxl(literal_operand, *reg_b);
            None
        }
        2 => {
            *reg_b = bst(combo_operand);
            None
        }
        3 => {
            *instruction_ptr = jnz(literal_operand, *reg_a, *instruction_ptr);
            None
        },
        4 => {
            *reg_b = bxc(*reg_b, *reg_c);
            None
        },
        5 => Some(out(combo_operand)),
        6 => {
            // The bdv instruction (opcode 6) works exactly like the adv instruction
            // except that the result is stored in the B register. (The numerator is still read from the A register.)
            *reg_b = adv(combo_operand, *reg_a);
            None
        },
        7 => {
            // The cdv instruction (opcode 7) works exactly like the adv instruction
            // except that the result is stored in the C register. (The numerator is still read from the A register.)
            *reg_c = adv(combo_operand, *reg_a);
            None
        },
        _ => {
            println!("Unknown opcode {}", opcode);
            None
        },

    }
}

fn part_one(input: &str) {
    let mut reg_a = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut program = Vec::new();

    let mut instruction_ptr: usize = 0;

    for line in input.lines() {
        if line.contains("A") {
            reg_a = line.split(':').last().unwrap().trim().parse::<i32>().unwrap();
        }
        else if line.contains("B") {
            reg_b = line.split(':').last().unwrap().trim().parse::<i32>().unwrap();
        }
        else if line.contains("C") {
            reg_c = line.split(':').last().unwrap().trim().parse::<i32>().unwrap();
        }
        else if line.contains("Program") {
            program = line
                .split(":")
                .nth(1)
                .unwrap_or("-999")
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
        }
    }

    println!("A: {}, B: {}, C: {}, Program: {:?}", reg_a, reg_b, reg_c, program);

    let mut program_output = Vec::new();
    while instruction_ptr < program.len() {
        if instruction_ptr == program.len() - 1 {
            break; // halt
        }

        let opcode = program[instruction_ptr];
        let literal_op = program[instruction_ptr + 1];
        let combo_op = interpret_combo_operand(program[instruction_ptr + 1], reg_a, reg_b, reg_c);

        if let Some(instruction_result) = perform_instruction(opcode,
                                                              literal_op,
                                                              combo_op,
                                                              &mut instruction_ptr,
                                                              &mut reg_a,
                                                              &mut reg_b,
                                                              &mut reg_c) {
            println!("Got output: {}", instruction_result);
            program_output.push(instruction_result);
        }

        println!("A: {}, B: {}, C: {}", reg_a, reg_b, reg_c);
        println!("{}", instruction_ptr);
    }
    println!("Program output: {:?}", program_output)
}

fn part_two(input: &str) -> i32 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(17);
    let example_input = utils::read_input_from_path("C:\\Documenten\\magic-repo\\advent_of_code_2024\\example_input\\day17.txt");

    let now = SystemTime::now();
    let ans = part_one(&input);
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());
    // println!("Part One: {}", ans);

    let now = SystemTime::now();
    // let ans_2 = part_two(&input);
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());
    // println!("Part One V2: {}", ans_2);

    Ok(())
}