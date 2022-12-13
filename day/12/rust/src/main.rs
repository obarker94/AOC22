use std::{collections::HashMap, isize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Mountain {
    grid: Vec<Vec<u8>>,
    start: (isize, isize),
    end: (isize, isize),
    height: isize,
    width: isize,
}
pub const DIR: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Mountain {
    fn new_mountain() -> Mountain {
        Mountain {
            grid: vec![vec![0; 10]; 10],
            start: (0, 0),
            end: (9, 9),
            height: 10,
            width: 10,
        }
    }
    fn read_file(&mut self, path: String) {
        let file = std::fs::read_to_string(path).expect("Unable to read file");
        let lines = file.lines().collect::<Vec<&str>>();

        for (row, line) in lines.iter().enumerate() {
            let mut gridline = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
            if let Some(start_point) = gridline.iter().position(|&c| c == b'S') {
                self.start = (row as isize, start_point as isize);
                gridline[start_point] = b'a';
            }
            if let Some(end_point) = gridline.iter().position(|&c| c == b'E') {
                self.end = (row as isize, end_point as isize);
                gridline[end_point] = b'z';
            }
            self.grid.push(gridline)
        }
        self.width = self.grid[0].len() as isize;
        self.height = self.grid.len() as isize;
    }

    fn get_surrounding_points(self, pos: (isize, isize)) -> Vec<(isize, isize)> {
        let ipos = (pos.0 as i32, pos.1 as i32);
        DIR.iter()
            .map(|d| (ipos.0 + d.0 as i32, ipos.1 + d.1 as i32))
            .filter(|pos| {
                pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width as i32 && pos.1 < self.height as i32
            })
            .map(|pos| (pos.0 as isize, pos.1 as isize))
            .collect()
    }
}

fn main() {
    let mut mountain = Mountain::new_mountain();
    mountain.read_file("test_input.txt".to_string());

    let mut to_visit = Vec::new();

    to_visit.extend(mountain.clone().get_surrounding_points(mountain.start));

    let mut shortest: HashMap<(isize, isize), isize> = HashMap::new();

    shortest.insert(mountain.start, 0);

    while let Some(loc) = to_visit.pop() {}

    // find
    // look for smllest value + 1
    // if smaller than current locations, path-dist update
    //update current location path dist
    // insert into to_vist locations around us

    println!("{:?}", mountain);
}
