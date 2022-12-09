#[derive(Debug)]
struct Command {
    name: String,
    value: u8,
}

impl Command {
    fn new(name: String, value: u8) -> Command {
        Command { name, value }
    }
}

struct Rope {
    head: Vec<usize>,
    tail: Vec<usize>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Vec::new(),
            tail: Vec::new(),
        }
    }

    fn move_right(number: usize) {
        for _ in 0..number {
            println!("Move right");
        }
    }
    fn move_left(number: usize) {
        for _ in 0..number {
            println!("Move left");
        }
    }
    fn move_up(number: usize) {
        for _ in 0..number {
            println!("Move up");
        }
        println!("move up");
    }
    fn move_down(number: usize) {
        for _ in 0..number {
            println!("Move down");
        }
    }
}

fn read_file(path: &String) -> Vec<Command> {
    let file = std::fs::read_to_string(&path).expect("Unable to read file");
    let lines = file
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut split_lines = Vec::new();

    for line in lines {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let new_command = Command::new(
            split_line[0].to_string(),
            split_line[1].parse::<u8>().unwrap(),
        );
        split_lines.push(new_command);
    }
    split_lines
}

fn main() {
    let time = std::time::Instant::now();
    let commands = read_file(&String::from("test_input.txt"));

    for command in commands {
        match command.name.as_str() {
            "R" => Rope::move_right(command.value as usize),
            "L" => Rope::move_left(command.value as usize),
            "U" => Rope::move_up(command.value as usize),
            "D" => Rope::move_down(command.value as usize),
            _ => println!("Unknown command"),
        }
    }

    let time_taken = time.elapsed();
    println!("Time taken: {:?}", time_taken);
}
