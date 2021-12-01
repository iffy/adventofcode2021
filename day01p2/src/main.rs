use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;
use std::collections::VecDeque;
use std::vec::Vec;

// fn sums(x: VecDeque<u32>) -> (u32, u32) {
//     return (x[0] + x[1] + x[2], x[1] + x[2] + x[3])
//     // I can't get this to work...
//     // let v = Vec::from(x);
//     // let a: u32 = (&v[1..3]).iter().sum();
//     // let b: u32 = (&v[2..4]).iter().sum();
//     // return (a, b);
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{}", &args[1]);
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
            let asum = slide[0] + slide[1] + slide[2];
            let bsum = slide[1] + slide[2] + slide[3];
            if bsum > asum {
                increases += 1;
            }
        }
    }
    println!("Increases: {}", increases);
}