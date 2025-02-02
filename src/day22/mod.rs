use std::{
    collections::HashMap,
    fs,
    ops::{BitAnd, BitOr, BitXor},
};
#[derive(Debug, Clone)]
struct Wire {
    instruction: Option<(String, Instruction, String)>,
    value: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    And,
    Or,
    Xor,
}

impl BitAnd for Wire {
    type Output = Option<bool>;

    fn bitand(self, rhs: Self) -> Self::Output {
        return self.value.and(rhs.value);
    }
}

impl BitOr for Wire {
    type Output = Option<bool>;

    fn bitor(self, rhs: Self) -> Self::Output {
        return self.value.or(rhs.value);
    }
}

impl BitXor for Wire {
    type Output = Option<bool>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        return self.value.xor(rhs.value);
    }
}

fn generate_gates(input: String) -> HashMap<String, Wire> {
    let mut result = HashMap::new();

    for line in input.lines() {
        if line.contains(':') {
            let name = line[0..=2].to_string();
            let value = Some(line.chars().last().unwrap() == '1');
            result.insert(
                name,
                Wire {
                    value,
                    instruction: None,
                },
            );
            continue;
        }

        if line.len() < 3 {
            continue;
        }

        let mut parts = line.split(' ');

        let value1 = parts.next().unwrap().to_string();
        let instruction = match parts.next().unwrap() {
            "XOR" => Instruction::Xor,
            "OR" => Instruction::Or,
            "AND" => Instruction::And,
            _ => panic!("Invalid Instruction"),
        };
        let value2 = parts.next().unwrap().to_string();
        let name = parts.last().unwrap().to_string();

        result.insert(
            name,
            Wire {
                instruction: Some((value1, instruction, value2)),
                value: None,
            },
        );
    }

    return result;
}

#[test]
fn test_input_parsing() {
    let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
        .to_string();

    let result = generate_gates(input);

    let x00 = result.get("x00").unwrap().value.unwrap();
    let x01 = result.get("x01").unwrap().value.unwrap();
    let x02 = result.get("x02").unwrap().value.unwrap();

    assert_eq!(x00, true);
    assert_eq!(x01, true);
    assert_eq!(x02, true);

    let y00 = result.get("y00").unwrap().value.unwrap();
    let y01 = result.get("y01").unwrap().value.unwrap();
    let y02 = result.get("y02").unwrap().value.unwrap();

    assert_eq!(y00, false);
    assert_eq!(y01, true);
    assert_eq!(y02, false);

    let z00 = &result.get("z00").unwrap().instruction;
    let z01 = &result.get("z01").unwrap().instruction;
    let z02 = &result.get("z02").unwrap().instruction;

    assert_eq!(
        z00,
        &Some(("x00".to_string(), Instruction::And, "y00".to_string()))
    );
    assert_eq!(
        z01,
        &Some(("x01".to_string(), Instruction::Xor, "y01".to_string()))
    );
    assert_eq!(
        z02,
        &Some(("x02".to_string(), Instruction::Or, "y02".to_string()))
    );
}

fn resolve_values(values: &mut HashMap<String, Wire>) {
    loop {
        let reference = values.clone();
        let mut has_no_more_nones = true;
        for (_, wire) in values.iter_mut() {
            if let None = wire.value {
                has_no_more_nones = false;
                if let Some(instruction) = &wire.instruction {
                    let value1 = reference.get(&instruction.0).unwrap().value;
                    let value2 = reference.get(&instruction.2).unwrap().value;

                    if value1.is_some() && value2.is_some() {
                        wire.value = match instruction.1 {
                            Instruction::And => Some(value1.unwrap() && value2.unwrap()),
                            Instruction::Or => Some(value1.unwrap() || value2.unwrap()),
                            Instruction::Xor => Some(value1.unwrap() ^ value2.unwrap()),
                        }
                    }
                }
            }
        }

        if has_no_more_nones {
            return;
        }
    }
}

#[test]
fn verify_resolved_values() {
    let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
        .to_string();

    let mut result = generate_gates(input);

    resolve_values(&mut result);

    let z00 = result.get("z00").unwrap().value.unwrap();
    let z01 = result.get("z01").unwrap().value.unwrap();
    let z02 = result.get("z02").unwrap().value.unwrap();

    assert_eq!(z00, false);
    assert_eq!(z01, false);
    assert_eq!(z02, true);
}

fn convert_result_to_number(values: HashMap<String, Wire>) -> usize {
    let mut total = 0;

    for (name, wire) in values {
        if !name.starts_with("z") {
            continue;
        }

        if let None = wire.value {
            panic!("values not yet resolved!");
        }

        if wire.value == Some(true) {
            let power_of_2 = name[1..].parse().unwrap();

            total += 2_usize.pow(power_of_2);
        }
    }

    return total;
}

#[test]
fn test_corect_output_after_resolution() {
    let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
        .to_string();

    let mut result = generate_gates(input);

    resolve_values(&mut result);

    let result = convert_result_to_number(result);

    assert_eq!(result, 4);
}

#[test]
fn test_sample_part_1() {
    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
        .to_string();

    let mut result = generate_gates(input);

    resolve_values(&mut result);

    let result = convert_result_to_number(result);

    assert_eq!(result, 2024);
}

pub fn day_22_part_1() {
    let input = fs::read_to_string("src/day22/input.txt").expect("day 22 input not present");
    let mut result = generate_gates(input);

    resolve_values(&mut result);

    let result = convert_result_to_number(result);

    println!("The final numbe the wires spell out is: {}", result);
}
