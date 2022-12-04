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

            for (_, left_char) in left.iter().enumerate() {
                for (_, right_char) in right.iter().enumerate() {
                    if left_char == right_char {
                        duplicate_item.push(*right_char);
                        break;
                    }
                }
            }
            duplicate_item.chars().nth(0).unwrap()
        })
        .collect::<Vec<char>>();

    let sum_of_duplicated_items = duplicated_items_in_rucksack
        .iter()
        .map(|c| alphabet_hashmap.get(c).unwrap())
        .sum::<usize>();
    println!("{:?}", sum_of_duplicated_items);

    // part 2 --- common items in backpack across 3 elves

    let group_of_three_backpacks = lines.chunks(3).collect::<Vec<&[&str]>>();

    let common_item_in_backpack = group_of_three_backpacks
        .iter()
        .map(|group| {
            let mut common_item = String::new();
            let mut group = group.iter().map(|line| line.chars().collect::<Vec<char>>());

            let first = group.next().unwrap();
            let second = group.next().unwrap();
            let third = group.next().unwrap();

            for (_, first_char) in first.iter().enumerate() {
                if second.contains(first_char) && third.contains(first_char) {
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
