use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{}", &args[1]);
    let file = File::open(&args[1]).unwrap();
    let mut lastvalue: u32 = 0;
    let mut started = false;
    let mut increases: u32 = 0;
    for line in BufReader::new(file).lines() {
        let thisvalue: u32 = line.unwrap().parse::<u32>().unwrap();
        if started && thisvalue > lastvalue {
            // println!("increase");
            increases += 1;
        }
        if !started {
            started = true;
        }
        lastvalue = thisvalue;
        // println!("{}", thisvalue);
    }
    println!("Increases: {}", increases);
}