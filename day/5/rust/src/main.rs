use std::fs;

#[derive(Debug, Clone)]
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

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = Vec::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    for c in 0..cols {
        let mut row = Vec::new();
        for r in 0..rows {
            row.push(matrix[r][c]);
        }
        transposed.push(row);
    }

    for row in transposed.iter_mut() {
        row.reverse();
        row.retain(|&c| c != ' ');
    }

    transposed
}

#[derive(Debug, Clone)]
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

    fn transform(&mut self) {
        let mut _towers = self.towers.clone();
        let mapped_towers = transpose(_towers);
        self.towers = mapped_towers;
    }

    fn move_crates(&self) -> Vec<char> {
        let mut towers_clone = self.towers.clone();
        for instruction in &self.instructions {
            let (mov, from, to) = (instruction.move_piece, instruction.from, instruction.to);

            let mut _ele_to_move: char = ' ';

            for _ in 0..mov {
                _ele_to_move = towers_clone[(from - 1) as usize].pop().unwrap();
                towers_clone[(to - 1) as usize].push(_ele_to_move);
            }
        }

        let mut answer = Vec::new();

        for tower in towers_clone.iter_mut() {
            let mut _tower = tower.clone();
            answer.push(_tower.pop().unwrap());
        }

        answer
    }

    fn turbo_crate_mover(&self) -> Vec<char> {
        let mut towers_clone = self.towers.clone();
        for instruction in &self.instructions {
            let (mov, from, to) = (instruction.move_piece, instruction.from, instruction.to);

            let mut _ele_to_move: char = ' ';
            let mut _eles_to_move: Vec<char> = Vec::new();

            for _ in 0..mov {
                _ele_to_move = towers_clone[(from - 1) as usize].pop().unwrap();
                _eles_to_move.push(_ele_to_move);
            }

            _eles_to_move.reverse();

            for ele in _eles_to_move {
                towers_clone[(to - 1) as usize].push(ele);
            }
        }

        let mut answer = Vec::new();

        for tower in towers_clone.iter_mut() {
            let mut _tower = tower.clone();
            answer.push(_tower.pop().unwrap());
        }

        answer
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

    unique_numbers
}

fn read_file(path: &str) -> FileContents {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut number_of_stacks: usize = 0;
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
    let file_contents = FileContents::new(towers, instructions);
    file_contents
}

fn main() {
    let start = std::time::Instant::now();

    let mut file = read_file("input.txt");
    file.transform();
    let part_1 = file.move_crates();
    let part_2 = file.turbo_crate_mover();
    println!("Part_1: {:?}", part_1);
    println!("Part_2: {:?}", part_2);

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
