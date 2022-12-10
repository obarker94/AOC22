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

#[derive(Debug)]
struct Coordinate {
    x: i8,
    y: i8,
}

impl Coordinate {
    fn new(x: i8, y: i8) -> Coordinate {
        Coordinate { x, y }
    }
}

struct Rope {
    head: Vec<Coordinate>,
    tail: Vec<Coordinate>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: vec![Coordinate::new(0, 0)],
            tail: vec![Coordinate::new(0, 0)],
        }
    }

    fn move_right(&mut self, number: usize) {
        let rope_len = self.head.len();
        let new_x = self.head[rope_len - 1].x + number as i8;
        self.head
            .push(Coordinate::new(new_x, self.head[rope_len - 1].y));
    }
    fn move_left(&mut self, number: usize) {
        let rope_len = self.head.len();
        let new_x = self.head[rope_len - 1].x - number as i8;
        self.head
            .push(Coordinate::new(new_x, self.head[rope_len - 1].y));
    }
    fn move_up(&mut self, number: usize) {
        let rope_len = self.head.len();
        let new_y = self.head[rope_len - 1].y + number as i8;
        self.head
            .push(Coordinate::new(self.head[rope_len - 1].x, new_y));
    }
    fn move_down(&mut self, number: usize) {
        let rope_len = self.head.len();
        let new_y = self.head[rope_len - 1].y - number as i8;
        self.head
            .push(Coordinate::new(self.head[rope_len - 1].x, new_y));
    }
    fn move_head(&mut self, command: Command) {
        match command.name.as_str() {
            "R" => self.move_right(command.value as usize),
            "L" => self.move_left(command.value as usize),
            "U" => self.move_up(command.value as usize),
            "D" => self.move_down(command.value as usize),
            _ => println!("Invalid command"),
        }
        println!("Head: {:?}", self.head.last());
    }

    fn move_tail(&mut self) {
        let head_len = self.head.len();
        let head_x = self.head[head_len - 1].x;
        let head_y = self.head[head_len - 1].y;
        let tail_len = self.tail.len();
        let tail_x = self.tail[tail_len - 1].x;
        let tail_y = self.tail[tail_len - 1].y;

        let x_diff = head_x - tail_x;
        let y_diff = head_y - tail_y;

        if x_diff != 0 && y_diff != 0 {
            println!("im diagonal");
            println!("x_diff: {}", x_diff);
            println!("y_diff: {}", y_diff);
        } else if head_x > tail_x {
            self.tail.push(Coordinate::new(tail_x + x_diff - 1, tail_y));
        } else if head_x < tail_x {
            self.tail.push(Coordinate::new(tail_x + x_diff + 1, tail_y));
        } else if head_y > tail_y {
            self.tail.push(Coordinate::new(tail_x, tail_y + y_diff - 1));
        } else if head_y < tail_y {
            self.tail.push(Coordinate::new(tail_x, tail_y + y_diff + 1));
        }
        println!("tail: {:?}", self.tail.last());
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

    let mut rope = Rope::new();

    for command in commands {
        rope.move_head(command);
        rope.move_tail();
    }

    let time_taken = time.elapsed();
    println!("Time taken: {:?}", time_taken);
}
