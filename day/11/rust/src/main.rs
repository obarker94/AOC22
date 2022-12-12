use std::fs;

#[derive(Debug)]
struct Operation {
    op: String,
    arg: String,
}

#[derive(Debug)]
struct Test {
    divisible_by: u8,
}

#[derive(Debug)]
struct Monkey {
    id: u8,
    items: Vec<u8>,
    operation: Operation,
    test: Test,
    pass: u8,
    fail: u8,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum MonkeyLine {
    id(u8),
    items(Vec<u8>),
    operation(Operation),
    test(Test),
    pass(u8),
    fail(u8),
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            items: Vec::new(),
            operation: Operation {
                op: String::new(),
                arg: String::new(),
            },
            test: Test { divisible_by: 0 },
            pass: 0,
            fail: 0,
        }
    }
}

fn parse_line(line: &Vec<&str>) -> MonkeyLine {
    let extracted_data_from_line = match line[0] {
        "Monkey" => {
            let number = line[1].split(":").collect::<Vec<&str>>()[0]
                .parse::<u8>()
                .unwrap();
            MonkeyLine::id(number)
        }
        "Starting" => {
            let mut items = Vec::new();
            for (i, item) in line.iter().enumerate() {
                if i > 1 {
                    let item = item.replace(",", "");
                    let value = item
                        .split(",")
                        .map(|s| s.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>();
                    items.push(value[0]);
                }
            }
            MonkeyLine::items(items)
        }
        "Operation:" => MonkeyLine::operation(Operation {
            op: line[line.len() - 2].to_string(),
            arg: line[line.len() - 1].to_string(),
        }),
        "Test:" => MonkeyLine::test(Test {
            divisible_by: line[line.len() - 1].parse::<u8>().unwrap(),
        }),
        "true:" => MonkeyLine::pass(line[line.len() - 1].parse::<u8>().unwrap()),
        "false:" => MonkeyLine::fail(line[line.len() - 1].parse::<u8>().unwrap()),

        _ => panic!("Invalid line"),
    };
    extracted_data_from_line
}

fn read_file(path: String) -> Vec<Monkey> {
    let file = fs::read_to_string(path).expect("Unable to read file");
    let mut lines = file.lines().collect::<Vec<&str>>();

    lines = lines.iter().map(|s| s.trim()).collect::<Vec<&str>>();
    lines.push("");

    let mut monkeys = Vec::new();
    let mut new_monkey = Vec::new();

    for line in lines {
        if line == "" {
            monkeys.push(new_monkey);
            new_monkey = Vec::new();
        } else {
            let mut _line = line.split(" ").collect::<Vec<&str>>();
            if _line[0] == "If" {
                _line.remove(0);
            }
            new_monkey.push(parse_line(&_line));
        }
    }

    let mut _monkeys = Vec::new();

    for _monkey in monkeys {
        let mut monkey = Monkey::new();
        for line in _monkey {
            match line {
                MonkeyLine::id(id) => monkey.id = id,
                MonkeyLine::items(items) => monkey.items = items,
                MonkeyLine::operation(operation) => monkey.operation = operation,
                MonkeyLine::test(test) => monkey.test = test,
                MonkeyLine::pass(pass) => monkey.pass = pass,
                MonkeyLine::fail(fail) => monkey.fail = fail,
            }
        }
        println!("{:?}", monkey);
        _monkeys.push(monkey);
    }

    _monkeys
}
fn main() {
    let timer = std::time::Instant::now();
    let test = read_file("test_input.txt".to_string());

    let timer = timer.elapsed();
    println!("{:?}", timer);
}
