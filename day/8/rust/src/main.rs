fn part_1(input: &str) -> usize {
    let mut cells = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|byte| byte.to_digit(10).unwrap() as i16)
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();
    cells.pop();
    let len = cells.len();
    let row_len = cells[0].len();
    let mut visible = 0;
    let mut maximums = Vec::new();
    let mut seen: Vec<Vec<i16>> = Vec::new();
    for i in 0..len {
        seen.push(Vec::new());
        let len = row_len;
        for _ in 0..len {
            seen[i].push(0);
        }
        maximums.push(-1);
    }
    for i in 0..len {
        for j in 0..row_len {
            if cells[i][j] > maximums[i] {
                maximums[i] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    maximums.clear();
    for _ in 0..len {
        maximums.push(-1);
    }
    for i in 0..len {
        for x in 0..row_len {
            let j = row_len - x - 1;
            if cells[i][j] > maximums[i] {
                maximums[i] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }

    maximums.clear();
    for _ in 0..row_len {
        maximums.push(-1);
    }
    for j in 0..row_len {
        for i in 0..len {
            if cells[i][j] > maximums[j] {
                maximums[j] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    maximums.clear();
    for _ in 0..row_len {
        maximums.push(-1);
    }
    for j in 0..row_len {
        for x in 0..len {
            let i = len - x - 1;
            if cells[i][j] > maximums[j] {
                maximums[j] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    visible
}

fn part_2(input: &str) -> usize {
    let mut cells = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|byte| byte.to_digit(10).unwrap() as i16)
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();
    cells.pop();
    let len = cells.len();
    let row_len = cells[0].len();
    let mut maximum = 0;
    for i in 1..len - 1 {
        for j in 1..row_len - 1 {
            let height = cells[i][j];
            let mut i2 = i;
            let mut j2 = j;
            let mut right = 0;
            let mut left = 0;
            let mut above = 0;
            let mut below = 0;
            while i2 > 0 {
                i2 -= 1;
                above += 1;
                if cells[i2][j] >= height {
                    i2 = 0;
                }
            }
            i2 = i;
            while i2 < len - 1 {
                i2 += 1;
                below += 1;
                if cells[i2][j] >= height {
                    i2 = len;
                }
            }
            while j2 > 0 {
                j2 -= 1;
                left += 1;
                if cells[i][j2] >= height {
                    j2 = 0;
                }
            }
            j2 = j;
            while j2 < row_len - 1 {
                j2 += 1;
                right += 1;
                if cells[i][j2] >= height {
                    j2 = row_len;
                }
            }
            if (left * right * above * below) > maximum {
                maximum = left * right * above * below;
            }
        }
    }

    maximum
}

fn main() {
    let time = Instant::now();
    let input = include_str!("./inputs/input.txt");
    println!("input, {:?}", input);
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
    println!("Time: {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT), 8);
    }
}
use std::time::Instant;


