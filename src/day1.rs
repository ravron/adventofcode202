use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day1() {
    let f = File::open("day1.txt").unwrap();
    let r = BufReader::new(f);

    let mut v: Vec<u32> = Vec::new();

    for line in r.lines() {
        let value: u32 = line.unwrap().parse().unwrap();
        v.push(value);
    }

    'outer: for i in &v {
        for j in &v {
            if i + j == 2020 {
                println!("{}", i * j);
                break 'outer;
            }
        }
    }

    'outer2: for i in &v {
        for j in &v {
            for k in &v {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    break 'outer2;
                }
            }
        }
    }
}