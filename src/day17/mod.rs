use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Computer {
    register_a: isize,
    register_b: isize,
    register_c: isize,
    program: Vec<isize>,
    execution_index: usize,
    output: Vec<isize>,
}

fn read_input() -> Computer {
    let input = fs::read_to_string("src/day17/input.txt").expect("error reading day 17 input");
    let mut input = input.lines();

    let register_a = input.next().unwrap();
    let register_b = input.next().unwrap();
    let register_c = input.next().unwrap();
    input.next();
    let program = input.next().unwrap();

    let number_regex = Regex::new(r"\d+").unwrap();

    let register_a = number_regex
        .find(register_a)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .expect("incorrect input format");
    let register_b = number_regex
        .find(register_b)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .expect("incorrect input format");
    let register_c = number_regex
        .find(register_c)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .expect("incorrect input format");

    let program = number_regex
        .find_iter(program)
        .map(|number_string| number_string.as_str().parse::<isize>().unwrap())
        .collect();

    return Computer {
        register_a,
        register_b,
        register_c,
        program,
        execution_index: 0,
        output: vec![],
    };
}

impl Computer {
    fn convert_to_combo_operand(&self, operand: isize) -> isize {
        match operand {
            0_isize..=3_isize => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("reserved instruction"),
            _ => panic!("instruction overflow"),
        }
    }

    fn print_output(&self) {
        let result = self
            .output
            .iter()
            .map(|numbner| numbner.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("output: {}", result)
    }

    fn execute_program(&mut self) {
        let program = self.program.clone();
        while let Some(opcode) = program.get(self.execution_index) {
            let current_execution_index = self.execution_index;
            match opcode {
                0 => self.adv(self.program[self.execution_index + 1]),
                1 => self.bxl(self.program[self.execution_index + 1]),
                2 => self.bst(self.program[self.execution_index + 1]),
                3 => self.jnz(self.program[self.execution_index + 1]),
                4 => self.bxc(self.program[self.execution_index + 1]),
                5 => self.out(self.program[self.execution_index + 1]),
                6 => self.bdv(self.program[self.execution_index + 1]),
                7 => self.cdv(self.program[self.execution_index + 1]),
                _ => panic!("invalid opcode at index {}", self.execution_index),
            }

            if current_execution_index == self.execution_index {
                self.execution_index += 2;
            }
        }

        self.print_output();
    }

    fn adv(&mut self, operand: isize) {
        let combo_operand = self.convert_to_combo_operand(operand);
        let numerator = self.register_a;
        let base: isize = 2;
        let denominator = base.pow(combo_operand as u32);

        let result = numerator / denominator;
        self.register_a = result;
    }

    fn bxl(&mut self, operand: isize) {
        let lhs = self.register_b;
        let rhs = operand;

        let result = lhs ^ rhs;
        self.register_b = result;
    }

    fn bst(&mut self, operand: isize) {
        let combo_operand = self.convert_to_combo_operand(operand);
        let result = combo_operand % 8;

        self.register_b = result;
    }

    fn jnz(&mut self, operand: isize) {
        if self.register_a == 0 {
            return;
        }

        self.execution_index = operand as usize;
    }

    fn bxc(&mut self, _operand: isize) {
        let lhs = self.register_b;
        let rhs = self.register_c;

        let result = lhs ^ rhs;
        self.register_b = result;
    }

    fn out(&mut self, operand: isize) {
        let combo_operand = self.convert_to_combo_operand(operand);
        let result = combo_operand % 8;
        self.output.push(result);
    }

    fn bdv(&mut self, operand: isize) {
        let combo_operand = self.convert_to_combo_operand(operand);
        let numerator = self.register_a;
        let base: isize = 2;
        let denominator = base.pow(combo_operand as u32);

        let result = numerator / denominator;
        self.register_b = result;
    }

    fn cdv(&mut self, operand: isize) {
        let combo_operand = self.convert_to_combo_operand(operand);
        let numerator = self.register_a;
        let base: isize = 2;
        let denominator = base.pow(combo_operand as u32);

        let result = numerator / denominator;
        self.register_c = result;
    }
}

pub fn day_17_part_1() {
    let mut input = read_input();

    input.execute_program();
}

pub fn day_17_part_2() {
    let mut input = read_input();

    let mut register_a = 0;
    let mut instructions = input.program.clone();

    instructions.reverse();

    for instruction in instructions {
        for index in 0..8 {
            input.register_a = register_a + index;
            input.output = vec![];
            input.execution_index = 0;
            input.execute_program();

            if input.output == input.program {
                register_a = register_a + index;
                break;
            }

            if *input.output.first().unwrap() == instruction {
                register_a = (register_a + index) * 8;
                break;
            }
        }
    }

    println!("{}", register_a);
}

#[test]
fn test_case_1() {
    let mut computer = Computer {
        register_a: 0,
        register_b: 0,
        register_c: 9,
        program: vec![2, 6],
        execution_index: 0,
        output: vec![],
    };

    computer.execute_program();

    assert_eq!(computer.register_b, 1);
}

#[test]
fn test_case_2() {
    let mut computer = Computer {
        register_a: 10,
        register_b: 0,
        register_c: 0,
        program: vec![5, 0, 5, 1, 5, 4],
        execution_index: 0,
        output: vec![],
    };

    computer.execute_program();

    assert_eq!(computer.output, vec![0, 1, 2]);
}

#[test]
fn test_case_3() {
    let mut computer = Computer {
        register_a: 2024,
        register_b: 0,
        register_c: 0,
        program: vec![0, 1, 5, 4, 3, 0],
        execution_index: 0,
        output: vec![],
    };

    computer.execute_program();

    assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
}

#[test]
fn test_case_4() {
    let mut computer = Computer {
        register_a: 0,
        register_b: 29,
        register_c: 0,
        program: vec![1, 7],
        execution_index: 0,
        output: vec![],
    };

    computer.execute_program();

    assert_eq!(computer.register_b, 26);
}

#[test]
fn test_case_5() {
    let mut computer = Computer {
        register_a: 0,
        register_b: 2024,
        register_c: 43690,
        program: vec![4, 0],
        execution_index: 0,
        output: vec![],
    };

    computer.execute_program();

    assert_eq!(computer.register_b, 44354);
}

#[test]
fn test_case_sample() {
    let mut computer = Computer {
        register_a: 729,
        register_b: 0,
        register_c: 0,
        program: vec![0, 1, 5, 4, 3, 0],
        execution_index: 0,
        output: vec![],
    };

    computer.execute_program();

    assert_eq!(computer.output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
}
#[test]
fn test_part_2_sample() {
    let mut computer = Computer {
        register_a: 117440,
        register_b: 0,
        register_c: 0,
        program: vec![0, 3, 5, 4, 3, 0],
        execution_index: 0,
        output: vec![],
    };

    computer.execute_program();

    assert_eq!(computer.output, computer.program);
}
