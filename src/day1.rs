use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day1() {
    let f = File::open("inputs/day1.txt").unwrap();
    let r = BufReader::new(f);

    let v: Vec<u32> = r.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    'outer: for i in &v {
        for j in &v {
            if i + j == 2020 {
                println!("part 1: {}", i * j);
                break 'outer;
            }
        }
    }

    'outer2: for i in &v {
        for j in &v {
            for k in &v {
                if i + j + k == 2020 {
                    println!("part 2: {}", i * j * k);
                    break 'outer2;
                }
            }
        }
    }
}