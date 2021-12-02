use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let mut depth = 0;
    let mut forward = 0;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let amount = parts[1].parse::<u32>().unwrap();
        match parts[0] {
            "forward" => forward += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => {},
        }
    }
    println!("depth {} forward {} answer {}", depth, forward, depth * forward);
}