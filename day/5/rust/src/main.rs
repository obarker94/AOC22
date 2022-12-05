fn split_game_data(data: Vec<String>, section_to_keep: i32) -> Vec<String> {
    let index = data.iter().position(|r| r == "").unwrap();
    if section_to_keep == 1 {
        let game_data = data[..index].to_vec();
        if game_data[0] == "" {
            game_data[1..].to_vec()
        } else {
            game_data
                .into_iter()
                .filter(|line| line.contains("["))
                .collect()
        }
    } else {
        let game_data = data[index..].to_vec();
        let first_element_removed = game_data[1..].to_vec();
        first_element_removed
    }
}

#[derive(Debug)]
struct Instruction {
    mov: i32,
    from: i32,
    to: i32,
}

impl Instruction {
    fn new(data: Vec<String>) -> Vec<Instruction> {
        let game_data = split_game_data(data, 0);
        let instructions_strings = game_data
            .iter()
            .map(|x| x.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let instructions = instructions_strings
            .iter()
            .map(|x| {
                let data = (x[1], x[3], x[5]);
                let (mov, from, to) = match data {
                    (mov, from, to) => (
                        mov.parse::<i32>().unwrap_or(0),
                        from.parse::<i32>().unwrap_or(0),
                        to.parse::<i32>().unwrap_or(0),
                    ),
                };

                Instruction { mov, from, to }
            })
            .collect::<Vec<Instruction>>();
        instructions
    }
}

#[derive(Debug)]
struct Tower<'a> {
    stack: Vec<&'a str>,
}

impl<'a> Tower<'a> {
    fn new() -> Tower<'a> {
        Tower { stack: vec![] }
    }

    fn push(&mut self, value: &'a str) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<&'a str> {
        self.stack.pop()
    }

    fn peek(&self) -> Option<&'a str> {
        self.stack.last().map(|x| *x)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[derive(Debug)]
struct Towers<'a> {
    towers: Vec<Tower<'a>>,
}

impl<'a> Towers<'a> {
    fn new() -> Towers<'a> {
        Towers {
            towers: vec![Tower::new()],
        }
    }

    fn push(&mut self, value: &'a str) {
        let last = self.towers.last_mut().unwrap();
        last.push(value);
    }

    fn pop(&mut self) -> Option<&'a str> {
        let last = self.towers.last_mut().unwrap();
        last.pop()
    }

    fn initialise(&mut self, value: &'a str) {
        self.towers.push(Tower::new());
        self.push(value);
    }
}

fn fetch_data(file: &str) -> Result<Vec<String>, &str> {
    let data_input = match std::fs::read_to_string(file) {
        Ok(data) => data.lines().map(|x| x.to_string()).collect::<Vec<String>>(),
        Err(_) => return Err("Could not read test_input.txt"),
    };
    return Ok(data_input);
}

fn get_towers(data: Vec<String>) -> () {
    let game_data = split_game_data(data, 1);
    let mut split_data = game_data
        .iter()
        .map(|x| x.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    split_data = split_data
        .iter()
        .map(|x| x[1..x.len() - 1].to_vec())
        .collect::<Vec<Vec<&str>>>();

    let mut raw_lines: Vec<Vec<&str>> = vec![];

    // remove the 4th and 8th element from split data
    // and push the result to parsed_lines

    for line in split_data {
        let mut new_line = vec![];
        for (index, element) in line.iter().enumerate() {
            if index != 3 && index != 7 {
                new_line.push(*element);
            }
        }
        raw_lines.push(new_line);
    }

    let mut parsed_lines: Vec<&str> = vec![];

    for line in raw_lines {
        for (i, element) in line.iter().enumerate() {
            if i % 3 == 1 {
                parsed_lines.push(element);
            }
        }
    }

    let mut chunked_data: Vec<Vec<&str>>;

    chunked_data = parsed_lines
        .chunks(3)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<&str>>>();

    chunked_data.reverse();

    let mut remapped_data: Vec<Vec<&str>> = vec![];

    for i in 0..3 {
        let mut new_line = vec![];
        for line in chunked_data.iter() {
            new_line.push(line[i]);
        }
        remapped_data.push(new_line);
    }

    println!("{:?}", remapped_data);
}

fn main() {
    let data = match fetch_data("test_input.txt") {
        Ok(data) => data,
        Err(e) => panic!("{}", e),
    };
    let data_clone = data.clone();
    let instructions = Instruction::new(data);
    let game_data = get_towers(data_clone);
    println!("{:?}", game_data);
}
