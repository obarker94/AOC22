use std::fs;
fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let file = fs::read_to_string("data.txt").unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let parsed_lines = lines
        .iter()
        .map(|x| {
            x.split(",")
                .map(|y| {
                    let mut nested_split = y.split("-").enumerate();
                    let (_, second) = nested_split.next().unwrap();
                    let first_value = second.parse::<u32>().unwrap();
                    let second_value = nested_split.next().unwrap().1.parse::<u32>().unwrap();

                    (first_value, second_value)
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .map(|x| {
            let first_section = x[0];
            let (f1, f2) = first_section;
            let second_section = x[1];
            let (s1, s2) = second_section;

            //part 1
            // (f1 <= s1 && s2 <= f2) || (s1 <= f1 && f2 <= s2)

            // part 2
            (f1 <= s1 && s1 <= f2) || (s1 <= f1 && f1 <= s2)
        })
        .collect::<Vec<_>>();

    let sum_trues = parsed_lines.iter().filter(|x| **x).count();
    println!("Sum of trues: {}", sum_trues);

    let elapsed = now.elapsed();
    println!("Time elapsed in function is: {:?}", elapsed);
}
