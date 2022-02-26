fn run(mut memory: Vec<i32>, input: i32) -> Option<i32> {
    let mut ip = 0;
    let mut output = None;

    loop {
        let opcode = memory[ip];
        let (_, s_mode, f_mode, instruction) = split_opcode(opcode);

        match instruction {
            1 | 2 => {
                let first_param = match f_mode {
                    0 => {
                        let addr = memory[ip + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[ip + 1],
                    _ => panic!("unknown param mode"),
                };

                let second_param = match s_mode {
                    0 => {
                        let addr = memory[ip + 2] as usize;
                        memory[addr]
                    }
                    1 => memory[ip + 2],
                    _ => panic!("unknown param mode"),
                };

                let addr = memory[ip + 3] as usize;

                if instruction == 1 {
                    memory[addr] = first_param + second_param;
                } else if instruction == 2 {
                    memory[addr] = first_param * second_param;
                }

                ip += 4;
            }
            3 => {
                let addr = memory[ip + 1] as usize;
                memory[addr] = input;
                ip += 2;
            }
            4 => {
                let value = match f_mode {
                    0 => {
                        let addr = memory[ip + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[ip + 1],
                    _ => panic!("unknown param mode"),
                };
                output = Some(value);
                ip += 2;
            }
            5 | 6 => {
                let first_param = match f_mode {
                    0 => {
                        let addr = memory[ip + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[ip + 1],
                    _ => panic!("unknown param mode"),
                };

                let second_param = match s_mode {
                    0 => {
                        let addr = memory[ip + 2] as usize;
                        memory[addr]
                    }
                    1 => memory[ip + 2],
                    _ => panic!("unknown param mode"),
                };

                if instruction == 5 && first_param != 0 {
                    ip = second_param as usize;
                } else if instruction == 6 && first_param == 0 {
                    ip = second_param as usize;
                } else {
                    ip += 3;
                }
            }
            7 | 8 => {
                let first_param = match f_mode {
                    0 => {
                        let addr = memory[ip + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[ip + 1],
                    _ => panic!("unknown param mode"),
                };

                let second_param = match s_mode {
                    0 => {
                        let addr = memory[ip + 2] as usize;
                        memory[addr]
                    }
                    1 => memory[ip + 2],
                    _ => panic!("unknown param mode"),
                };

                let addr = memory[ip + 3] as usize;

                if instruction == 7 {
                    if first_param < second_param {
                        memory[addr as usize] = 1;
                    } else {
                        memory[addr as usize] = 0;
                    }
                } else if instruction == 8 {
                    if first_param == second_param {
                        memory[addr as usize] = 1;
                    } else {
                        memory[addr as usize] = 0;
                    }
                }

                ip += 4;
            }
            99 => break,
            _ => panic!("unknown opcode {}", opcode),
        }
    }

    output
}

fn main() {
    let input = include_str!("../input");
    //let input = "1002,4,3,4,33";
    let rom = parse(input);

    let output_1 = run(rom.clone(), 1);

    println!("Part 1: {}", output_1.unwrap());
    let output_2 = run(rom.clone(), 5);
    println!("Part 2: {}", output_2.unwrap());
}

fn parse(input: &str) -> Vec<i32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn split_opcode(opcode: i32) -> (i32, i32, i32, i32) {
    let ins = opcode % 100;
    let third = (opcode / 10000) % 10;
    let second = (opcode / 1000) % 10;
    let first = (opcode / 100) % 10;

    (third, second, first, ins)
}
