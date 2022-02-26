use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum CPUState {
    Running,
    Halted,
    Terminated,
}

pub struct CPU {
    ip: usize,
    memory: Vec<i32>,
    pub input: VecDeque<i32>,
    pub output: i32,
    state: CPUState,
}

impl CPU {
    pub fn new() -> CPU {
        let ip = 0;
        let memory = Vec::new();
        let input = VecDeque::new();
        let output = 0;
        let state = CPUState::Running;

        CPU { ip, memory, input, output, state }
    }

    pub fn load_rom(&mut self, rom: Vec<i32>) {
        self.memory = rom;
        self.input.clear();
        self.output = 0;
        self.ip = 0;
        self.state = CPUState::Running;
    }

    fn get_param(&self, position: usize, mode: i32) -> i32 {
        match mode {
            0 => {
                let addr = self.memory[position] as usize;
                self.memory[addr]
            }
            1 => self.memory[position],
            _ => panic!("unknown param mode")
        }
    }

    pub fn cycle(&mut self) {
        if self.state != CPUState::Running {
            panic!("tried to cycle not running machine");
        }

        let opcode = self.memory[self.ip];
        let (_, s_mode, f_mode, instruction) = split_opcode(opcode);

        match instruction {
            1 | 2 | 7 | 8 => {
                let first_param = self.get_param(self.ip + 1, f_mode);
                let second_param = self.get_param(self.ip + 2, s_mode);

                let addr = self.memory[self.ip + 3] as usize;

                if instruction == 1 {
                    self.memory[addr] = first_param + second_param;
                } else if instruction == 2 {
                    self.memory[addr] = first_param * second_param;
                } else if instruction == 7 {
                    if first_param < second_param {
                        self.memory[addr as usize] = 1;
                    } else {
                        self.memory[addr as usize] = 0;
                    }
                } else if instruction == 8 {
                    if first_param == second_param {
                        self.memory[addr as usize] = 1;
                    } else {
                        self.memory[addr as usize] = 0;
                    }
                }

                self.ip += 4;
            }
            3 => {
                let addr = self.memory[self.ip + 1] as usize;
                self.memory[addr] = self.input.pop_front().expect("tried to get input but didnt get any");
                self.ip += 2;
            }
            4 => {
                let value = self.get_param(self.ip + 1, f_mode);
                self.output = value;
                self.ip += 2;
                self.state = CPUState::Halted;
            }
            5 | 6 => {
                let first_param = self.get_param(self.ip + 1, f_mode);
                let second_param = self.get_param(self.ip + 2, s_mode);

                if instruction == 5 && first_param != 0 {
                    self.ip = second_param as usize;
                } else if instruction == 6 && first_param == 0 {
                    self.ip = second_param as usize;
                } else {
                    self.ip += 3;
                }
            }
            99 => self.state = CPUState::Terminated,
            _ => panic!("unknown opcode {}", opcode),
        }
    }

    pub fn run(&mut self) {
        while self.state == CPUState::Running { self.cycle(); }
    }

    pub fn terminated(&self) -> bool { return self.state == CPUState::Terminated; }

    pub fn halted(&self) -> bool { return self.state == CPUState::Halted; }

    pub fn resume(&mut self) {
        if self.state == CPUState::Halted {
            self.state = CPUState::Running;
        } else {
            panic!("tried to resume not halted machine");
        }
    }
}

fn split_opcode(opcode: i32) -> (i32, i32, i32, i32) {
    let ins = opcode % 100;
    let third = (opcode / 10000) % 10;
    let second = (opcode / 1000) % 10;
    let first = (opcode / 100) % 10;

    (third, second, first, ins)
}