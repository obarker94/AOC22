#[derive(Debug)]

struct Instruction {
    mov: i32,
    from: i32,
    to: i32,
}

impl Instruction {
    fn new(mov: i32, from: i32, to: i32) -> Instruction {
        Instruction { mov, from, to }
    }
}

fn line_parser(line: &str) -> Result<Instruction, &str> {
    let data_input = match std::fs::read_to_string("test_input.txt") {
        Ok(data) => data,
        Err(_) => return Err("Could not read test_input.txt"),
    };
    let test_lines = data_input.lines().collect::<Vec<_>>();
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

    println!("{:?}", data);
}
