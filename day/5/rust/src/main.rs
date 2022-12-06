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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

    fn remove_x_towers(&mut self, x: usize) {
        self.towers.truncate(x);
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
    println!("Data input: {:?}", data_input);
    return Ok(data_input);
}

fn get_towers(data: Vec<String>, instructions: Vec<Instruction>) -> () {
    let game_data = split_game_data(data, 1);
    let split_data = &game_data
        .iter()
        .map(|x| x.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let purged_nulls = &split_data
        .iter()
        .map(|x| x[1..x.len() - 1].to_vec())
        .collect::<Vec<Vec<&str>>>();

    let mut raw_lines: Vec<Vec<&str>> = vec![];

    for line in purged_nulls {
        let mut new_line = vec![];
        for (index, element) in line.iter().enumerate() {
            if index != 3
                && index != 7
                && index != 11
                && index != 15
                && index != 19
                && index != 23
                && index != 27
                && index != 31
            {
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
        .chunks(9)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<&str>>>();

    chunked_data.reverse();
    let mut remapped_data: Vec<Vec<&str>> = vec![];

    for i in 0..9 {
        let mut new_line = vec![];
        for line in chunked_data.iter() {
            new_line.push(line[i]);
        }
        remapped_data.push(new_line);
    }

    let mut towers = Towers::new();

    for line in remapped_data {
        for (i, element) in line.iter().enumerate() {
            if i == 0 {
                towers.initialise(element);
            } else {
                if (element != &" ") {
                    towers.push(element);
                }
            }
        }
    }

    towers.towers.remove(0);

    for instruction in instructions {
        let (mov, from, to) = (instruction.mov, instruction.from, instruction.to);

        let mut ele_to_move = "";
        let mut eles_to_move: Vec<&str> = vec![];

        for _ in 0..mov {
            ele_to_move = towers.towers[(from - 1) as usize].pop().unwrap();
            eles_to_move.push(ele_to_move);
        }

        eles_to_move.reverse();

        for ele in eles_to_move {
            towers.towers[(to - 1) as usize].push(ele);
        }

        // for _ in 0..mov {
        //     ele_to_move = towers.towers[(from - 1) as usize].pop().unwrap();
        //     towers.towers[(to - 1) as usize].push(ele_to_move);
        // }

        println!("mov: {}, from: {}, to: {}", mov, from, to);
        println!("{:?}", towers);
    }

    let answer = &towers
        .towers
        .iter()
        .map(|x| x.peek().unwrap())
        .collect::<Vec<&str>>();

    println!("{:?}", answer);
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let data = match fetch_data("input.txt") {
        Ok(data) => data,
        Err(e) => panic!("{}", e),
    };
    let data_clone = data.clone();
    let instructions = Instruction::new(data);
    let game_data = get_towers(data_clone, instructions);
    let elapsed = now.elapsed();
    println!("Time elapsed in function is: {:?}", elapsed);
}
