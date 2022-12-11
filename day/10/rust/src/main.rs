use std::fs;

#[derive(Debug, Clone)]
struct Command {
    operation: String,
    value: Option<i32>,
}

impl Command {
    fn new(operation: String, value: Option<i32>) -> Command {
        Command { operation, value }
    }
}

#[derive(Debug, Clone)]
struct ElfComputer {
    program: Vec<Command>,
    command_number: usize,
    cycle: usize,
    signal: i32,
}

impl ElfComputer {
    fn new(program: Vec<Command>) -> ElfComputer {
        ElfComputer {
            program,
            command_number: 0,
            cycle: 0,
            signal: 1,
        }
    }

    fn run(&mut self, max_cycles: usize) -> i32 {
        self.cycle = 0;
        while self.cycle < max_cycles {
            let command = self.program[self.command_number].clone();
            match command.operation.as_str() {
                "addx" => {
                    self.cycle += 1;
                    for _ in 0..1 {
                        if self.cycle < max_cycles {
                            self.cycle += 1;
                        } else {
                            break;
                        }
                    }

                    self.command_number += 1;
                    if self.cycle == max_cycles {
                        break;
                    }
                    self.signal += command.value.unwrap();
                }
                "noop" => {
                    self.cycle += 1;
                    self.command_number += 1;
                }
                _ => panic!("Unknown operation"),
            }
            if self.command_number == self.program.len() {
                break;
            }
        }
        self.signal = self.signal * max_cycles as i32;
        self.signal
    }
}

fn read_file(path: String) -> Vec<Command> {
    let file = fs::read_to_string(path).expect("Incorrect file type");
    let lines = file
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let operation = match split.next() {
                Some(op) => op.to_string(),
                None => panic!("Incorrect file format"),
            };
            let value = match split.next() {
                Some(val) => Some(val.parse::<i32>().unwrap()),
                None => None,
            };
            Command::new(operation, value)
        })
        .collect::<Vec<Command>>();
    lines
}

fn main() {
    let time = std::time::Instant::now();
    let lines = read_file("input.txt".to_string());
    let cycles_to_analyze = vec![20, 60, 100, 140, 180, 220];

    let mut sum = 0;

    for cycles in cycles_to_analyze {
        let mut computer = ElfComputer::new(lines.clone());
        let signal = computer.run(cycles);
        sum += signal;
        println!("Signal after {} cycles: {}", cycles, signal);
    }

    println!("Sum of signals: {}", sum);
    let elapsed = time.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20_lines() {
        let lines = read_file("test_input.txt".to_string());
        let mut computer = ElfComputer::new(lines);

        computer.run(20);
        assert_eq!(computer.signal, 420);
    }

    #[test]
    fn test_60_lines() {
        let lines = read_file("test_input.txt".to_string());
        let mut computer = ElfComputer::new(lines);

        computer.run(60);
        assert_eq!(computer.signal, 1140);
    }

    #[test]
    fn test_100_lines() {
        let lines = read_file("test_input.txt".to_string());
        let mut computer = ElfComputer::new(lines);

        computer.run(100);
        assert_eq!(computer.signal, 1800);
    }
    #[test]
    fn test_140_lines() {
        let lines = read_file("test_input.txt".to_string());
        let mut computer = ElfComputer::new(lines);

        computer.run(140);
        assert_eq!(computer.signal, 2940);
    }
    #[test]
    fn test_180_lines() {
        let lines = read_file("test_input.txt".to_string());
        let mut computer = ElfComputer::new(lines);
        computer.run(180);
        assert_eq!(computer.signal, 2880);
    }
    #[test]
    fn test_220_lines() {
        let lines = read_file("test_input.txt".to_string());
        let mut computer = ElfComputer::new(lines);

        computer.run(220);
        assert_eq!(computer.signal, 3960);
    }
}
