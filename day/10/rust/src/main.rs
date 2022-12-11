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
        let mut cycles = 0;
        while cycles < max_cycles {
            let command = self.program[self.command_number].clone();
            println!("Cycle: {}, command: {:?}", cycles, command);
            match command.operation.as_str() {
                "addx" => {
                    cycles += 1;
                    for _ in 0..1 {
                        if cycles < max_cycles {
                            cycles += 1;
                        } else {
                            break;
                        }
                    }

                    self.command_number += 1;
                    if cycles == max_cycles {
                        break;
                    }
                    self.signal += command.value.unwrap();
                }
                "noop" => {
                    cycles += 1;
                    self.command_number += 1;
                    println!("Cycle: {} - noop", cycles);
                }
                _ => panic!("Unknown operation"),
            }
            self.cycle += 1;
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
    let lines = read_file("test_input.txt".to_string());
    let mut computer = ElfComputer::new(lines);
    computer.run(220);
    println!("Signal: {}", computer.signal);
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
