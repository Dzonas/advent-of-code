struct CPU {
    rom: Vec<usize>,
    memory: Vec<usize>,
}

impl CPU {
    fn new(rom: Vec<usize>) -> CPU {
        let memory = rom.clone();
        CPU { rom, memory }
    }

    fn set_input(&mut self, noun: usize, verb: usize) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    fn run(&mut self) {
        let mut ip = 0;
        loop {
            match self.memory[ip] {
                1 | 2 => {
                    let left_addr = self.memory[ip + 1];
                    let right_addr = self.memory[ip + 2];
                    let out_addr = self.memory[ip + 3];

                    if self.memory[ip] == 1 {
                        self.memory[out_addr] = self.memory[left_addr] + self.memory[right_addr];
                    } else {
                        self.memory[out_addr] = self.memory[left_addr] * self.memory[right_addr];
                    }
                }
                99 => break,
                _ => panic!("unknown opcode"),
            }

            ip += 4;
        }
    }

    fn reset(&mut self) {
        self.memory = self.rom.clone();
    }
}

fn main() {
    let input = include_str!("../input");
    let codes: Vec<usize> = input
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut cpu = CPU::new(codes);
    cpu.set_input(12, 2);
    cpu.run();

    println!("Part 1: {}", cpu.memory[0]);

    let mut noun = 100;
    let mut verb = 100;

    'outer: while noun > 0 {
        noun -= 1;
        verb = 99;

        while verb > 0 {
            verb -= 1;
            cpu.reset();
            cpu.set_input(noun, verb);
            cpu.run();

            if cpu.memory[0] == 19690720 {
                break 'outer;
            }
        }
    }

    println!("Part 2: {}, {}", noun, verb);
}
