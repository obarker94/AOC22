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

        println!("{:?}", instructions);

        instructions
    }
}

fn line_parser(line: &str) -> Result<Instruction, &str> {
    let test = Instruction {
        mov: 0,
        from: 0,
        to: 0,
    };
    return Ok(test);
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
    let parsed_data = Instruction::new(data);
}
