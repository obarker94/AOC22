use std::collections::HashMap;

// todo result with anyhow

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let mut alphabet_hashmap = HashMap::new();

    for (i, c) in alphabet.iter().enumerate() {
        alphabet_hashmap.insert(c, i + 1);
    }

    let data = std::fs::read_to_string("data.txt").expect("Error reading file");
    let lines = data.lines().collect::<Vec<&str>>();
    let parsed_lines = lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    // part 1 --- duplicated items in backpack

    let duplicated_items_in_rucksack = parsed_lines
        .iter()
        .map(|line| {
            let (left, right) = line;
            let left = left.chars().collect::<Vec<char>>();
            let right = right.chars().collect::<Vec<char>>();

            let mut duplicate_item = String::new();

            for (left_char, right_char) in left.iter().zip(right.iter()) {
                if left_char == right_char {
                    duplicate_item.push(*right_char);
                }
            }

            duplicate_item
        })
        .collect::<Vec<String>>();

    let sum_of_duplicated_items = duplicated_items_in_rucksack
        .iter()
        .map(|item| {
            item.chars()
                .map(|c| alphabet_hashmap.get(&c).unwrap())
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{:?}", sum_of_duplicated_items);

    // part 2 --- common items in backpack across 3 elves

    let group_of_three_backpacks = lines.chunks(3).collect::<Vec<&[&str]>>();

    let common_item_in_backpack = group_of_three_backpacks
        .iter()
        .map(|group| {
            let mut common_item = String::new();
            let (first_backpack, second_backpack, third_backpack) = (
                group[0].chars().collect::<Vec<char>>(),
                group[1].chars().collect::<Vec<char>>(),
                group[2].chars().collect::<Vec<char>>(),
            );

            for (_, first_char) in first_backpack.iter().enumerate() {
                if second_backpack.contains(first_char) && third_backpack.contains(first_char) {
                    common_item.push(*first_char);
                    break;
                }
            }

            common_item.chars().nth(0).unwrap()
        })
        .collect::<Vec<char>>();

    let sum_of_common_items = common_item_in_backpack
        .iter()
        .map(|c| alphabet_hashmap.get(c).unwrap())
        .sum::<usize>();

    println!("{:?}", sum_of_common_items);
    let elapsed = now.elapsed();
    println!("Time elapsed in function is: {:?}", elapsed);
}
