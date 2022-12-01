use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lines = contents.lines().collect::<Vec<&str>>();
    let mut elfs: Vec<Vec<i32>> = vec![vec![]; 1];
    let mut number_of_elfs = 0;

    for line in lines.iter_mut() {
        let words = &mut line.split_whitespace().collect::<Vec<&str>>();
        words.reverse();
        if words.len() > 0 {
            elfs[number_of_elfs].push(words.pop().unwrap().parse::<i32>().unwrap());
        } else {
            elfs.push(vec![]);
            number_of_elfs += 1;
        }
    }

    let mut summed_elf_food = vec![];
    let mut max_presents = 0;

    for elf in elfs.iter() {
        let mut presents = 0;
        for house in elf.iter() {
            presents += house * 10;
        }
        summed_elf_food.push(presents);
        if presents > max_presents {
            max_presents = presents;
        }
    }

    summed_elf_food.sort();
    summed_elf_food.reverse();
    summed_elf_food.truncate(3);

    let top_three_fat_elfs = summed_elf_food.iter().fold(0, |sum, x| sum + x);

    println!("Top 3 elves have {} calories", top_three_fat_elfs);
}
