use std::thread;

fn read_file(path: String) -> Vec<String> {
    let file = std::fs::read_to_string(path).expect("Unable to read file");
    let lines = file.lines().map(|s| s.to_string()).collect();
    lines
}

fn is_command(line: &String) -> bool {
    line.starts_with("$")
}

fn is_dir(line: &String) -> bool {
    line.starts_with("dir")
}

fn execute_command(line: &String) {
    println!("Executing command: {}", line);
}

fn process_commands(lines: Vec<String>) {
    println!("Processing commands");
    for line in lines {
        if is_command(&line) {
            execute_command(&line);
        } else if is_dir(&line) {
            println!("Found dir");
        } else {
            println!("Found line: {}", line);
        }
    }
}

fn main() {
    let lines = read_file("test_input.txt".to_string());
    process_commands(lines);
}
