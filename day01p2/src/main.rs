use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;
use std::collections::VecDeque;
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let mut slide: VecDeque<u32> = VecDeque::new();
    let mut increases: u32 = 0;
    for line in BufReader::new(file).lines() {
        let thisvalue: u32 = line.unwrap().parse::<u32>().unwrap();
        slide.push_back(thisvalue);
        if slide.len() > 4 {
            slide.pop_front();
        }
        if slide.len() == 4 {
            let asum: u32 = slide.make_contiguous()[0..2].iter().sum();
            let bsum: u32 = slide.make_contiguous()[1..3].iter().sum();
            if bsum > asum {
                increases += 1;
            }
        }
    }
    println!("Increases: {}", increases);
}