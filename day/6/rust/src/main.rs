use std::fs;

fn read_file(path: &str) -> String {
    let file = fs::read_to_string(path).expect("Unable to read file");
    let lines = file.lines().collect::<Vec<&str>>();
    let mut result = String::new();
    for line in lines {
        result.push_str(line);
    }
    result
}

#[derive(Debug, Clone)]
struct RadioStream {
    stream: Vec<char>,
}

impl RadioStream {
    fn new(path_name: String) -> RadioStream {
        let data = read_file(&path_name);
        let chars = data.chars().collect::<Vec<char>>();
        RadioStream { stream: chars }
    }
}

fn byte_checker(packet: &Vec<char>) -> bool {
    let mut unique = true;
    for i in 0..packet.len() {
        for j in 0..packet.len() {
            if i != j {
                if packet[i] == packet[j] {
                    unique = false;
                }
            }
        }
    }
    unique
}

fn packet_checker(stream: &RadioStream, packet_size: usize) -> i32 {
    let mut _packet_to_check: Vec<char> = Vec::new();
    _packet_to_check = stream.stream[0..packet_size].to_vec();

    for i in 0..stream.stream.len() {
        if i + packet_size < stream.stream.len() {
            _packet_to_check = stream.stream[i..i + packet_size].to_vec();
            if byte_checker(&_packet_to_check) {
                return i as i32 + packet_size as i32;
            }
        }
    }
    return 0;
}

fn main() {
    let start = std::time::Instant::now();

    let stream = RadioStream::new("input.txt".to_string());
    let part_1 = packet_checker(&stream, 4);
    let part_2 = packet_checker(&stream, 14);
    let duration = start.elapsed();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn char_byte_14() {
        let stream = RadioStream::new("test_input.txt".to_string());
        assert_eq!(19, packet_checker(&stream, 14));
    }

    #[test]
    fn char_byte_4() {
        let stream = RadioStream::new("test_input.txt".to_string());
        assert_eq!(7, packet_checker(&stream, 4));
    }
}
