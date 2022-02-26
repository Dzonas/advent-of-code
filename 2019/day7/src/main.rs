use itertools::Itertools;

mod intcode;

use intcode::CPU;

fn main() {
    let input = include_str!("../input");
    let rom = parse(input);
    let part1_output = part1(&rom);
    let part2_output = part2(&rom);

    println!("Part 1: {}", part1_output);
    println!("Part 2: {}", part2_output);
}

fn part2(rom: &Vec<i32>) -> i32 {
    let mut cpus = Vec::new();
    let mut max_output = std::i32::MIN;

    for _ in 0..5 {
        cpus.push( CPU::new());
    }

    for phase in (5..10).permutations(5) {
        // Load rom into the cpu and add phase to the input
        for (cpu, ph) in cpus.iter_mut().zip(&phase) {
            cpu.load_rom(rom.clone());
            cpu.input.push_back(*ph);
        }

        let mut input = 0; // Initial signal is 0
        // Run until all amplifiers are terminated
        while !cpus.iter().all(|v| v.terminated()) {
            // For each cpu cycle until amplifier sets output (got to instr. 4)
            // then pass output from one amplifier to another's input.
            // After that resume the machine if it's still running.
            for cpu in &mut cpus {
                cpu.input.push_back(input);
                while !cpu.halted() && !cpu.terminated() {
                    cpu.cycle();
                }
                input = cpu.output;

                if !cpu.terminated() {
                    cpu.resume();
                }
            }
        }

        if cpus[4].output > max_output {
            max_output = cpus[4].output;
        }
    }

    max_output
}

fn part1(rom: &Vec<i32>) -> i32 {
    let mut cpu = CPU::new();
    let mut max_output = std::i32::MIN;

    for phase in (0..5).permutations(5) {
        let mut amp_input = 0;

        for ph in phase.iter() {
            cpu.load_rom(rom.clone());
            cpu.input.push_back(*ph);
            cpu.input.push_back(amp_input);
            cpu.run();
            amp_input = cpu.output;
        }

        if amp_input > max_output {
            max_output = amp_input;
        }
    }

    max_output
}

fn parse(input: &str) -> Vec<i32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}
