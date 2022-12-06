use std::fs;

#[derive(Debug)]
struct Instructions {
    move_piece: i32,
    from: i32,
    to: i32,
}

impl Instructions {
    fn new(move_piece: i32, from: i32, to: i32) -> Self {
        Self {
            move_piece,
            from,
            to,
        }
    }
}

#[derive(Debug)]
struct FileContents {
    towers: Vec<Vec<char>>,
    instructions: Vec<Instructions>,
}

impl FileContents {
    fn new(towers: Vec<Vec<char>>, instructions: Vec<Instructions>) -> Self {
        FileContents {
            towers,
            instructions,
        }
    }
}

fn parse_instruction(line: Vec<String>, index_of_numbers: &Vec<usize>) -> Vec<i32> {
    let mut numbers = Vec::new();

    for i in 0..index_of_numbers.len() {
        if i == 0 && index_of_numbers[i] + 1 == index_of_numbers[i + 1] {
            let merged_number = format!(
                "{}{}",
                line[index_of_numbers[i]],
                line[index_of_numbers[i + 1]]
            );
            numbers.push(merged_number.parse::<i32>().unwrap());
        } else if i == 0 && index_of_numbers[1] + 1 != index_of_numbers[i + 1] {
            numbers.push(line[index_of_numbers[i]].parse::<i32>().unwrap());
        } else if i + 1 < index_of_numbers.len()
            && index_of_numbers[i] + 1 == index_of_numbers[i + 1]
        {
            let merged_number = format!(
                "{}{}",
                line[index_of_numbers[i]],
                line[index_of_numbers[i + 1]]
            );
            numbers.push(merged_number.parse::<i32>().unwrap());
        } else {
            if index_of_numbers[i] - 1 != index_of_numbers[i - 1] {
                numbers.push(line[index_of_numbers[i]].parse::<i32>().unwrap());
            } else {
                let merged_number = format!(
                    "{}{}",
                    line[index_of_numbers[i - 1]],
                    line[index_of_numbers[i]]
                );
                numbers.push(merged_number.parse::<i32>().unwrap());
            }
        }
    }

    let mut unique_numbers = Vec::new();
    if numbers.len() > 3 {
        for number in numbers {
            if !unique_numbers.contains(&number) {
                unique_numbers.push(number);
            }
        }
    } else {
        unique_numbers = numbers;
    }

    println!("unique_numbers: {:?}", unique_numbers);

    unique_numbers
}

fn read_file(path: &str) -> FileContents {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut number_of_stacks: usize = 0;
    println!("Number of stacks: {}", number_of_stacks);
    let mut instructions: Vec<Instructions> = Vec::new();
    let mut towers = Vec::new();
    for line in lines {
        let mut line_with_whitespace = line.split("").collect::<Vec<&str>>();
        line_with_whitespace.retain(|&x| x != "");
        line_with_whitespace.push(" ");
        if number_of_stacks == 0 {
            number_of_stacks = line_with_whitespace.len() / 4;
        }
        let mut tower = Vec::new();

        if line_with_whitespace[0] != "m" {
            for i in 0..line_with_whitespace.len() {
                if (i + 1) % 4 == 0 && i >= 2 {
                    tower.push(line_with_whitespace[i - 2].chars().next().unwrap());
                }
            }
            towers.push(tower);
        } else {
            let mut index_of_numbers = Vec::new();
            for i in 0..line_with_whitespace.len() {
                if line_with_whitespace[i].parse::<i32>().is_ok() {
                    index_of_numbers.push(i);
                }
            }

            let mut line_with_whitespace_as_strings = Vec::new();
            for i in 0..line_with_whitespace.len() {
                line_with_whitespace_as_strings.push(line_with_whitespace[i].to_string());
            }

            let output = parse_instruction(line_with_whitespace_as_strings, &index_of_numbers);
            let instruction = Instructions::new(output[0], output[1], output[2]);
            instructions.push(instruction);
        }
    }
    towers.pop();
    towers.pop();
    println!("Number of stacks: {}", number_of_stacks);

    let file_contents = FileContents::new(towers, instructions);
    file_contents
}

fn main() {
    let start = std::time::Instant::now();

    let file = read_file("test_input.txt");

    let duration = start.elapsed();
    println!("{:?}", file);
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

