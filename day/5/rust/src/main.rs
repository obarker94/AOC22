#[derive(Debug)]
struct Instruction {
    mov: i32,
    from: i32,
    to: i32,
}

impl Instruction {
    fn new(data: Vec<String>) -> Vec<Instruction> {
        let index = data.iter().position(|r| r == "").unwrap();
        let instructions = data.split_at(index).1;
        let first_element_removed = instructions[1..].to_vec();
        let instructions_strings = first_element_removed
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
}

fn fetch_data(file: &str) -> Result<Vec<String>, &str> {
    let data_input = match std::fs::read_to_string(file) {
        Ok(data) => data.lines().map(|x| x.to_string()).collect::<Vec<String>>(),
        Err(_) => return Err("Could not read test_input.txt"),
    };
    return Ok(data_input);
}

fn main() {
    let data = match fetch_data("test_input.txt") {
        Ok(data) => data,
        Err(e) => panic!("{}", e),
    };
    let instructions = Instruction::new(data);
    let mut game = Towers::new();
    game.towers.push(Tower::new());
    game.towers[0].push("a");
    game.towers[0].push("b");
    game.towers[0].push("c");
    game.towers[1].push("1");
    game.towers[1].push("2");
    game.towers[1].push("3");

    println!("{:?}", game);
}
