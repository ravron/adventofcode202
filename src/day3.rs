use std::fs::File;
use std::io::{BufReader, BufRead};

struct Field {
    lines: Vec<bool>,
    n: usize,
    m: usize,
}

pub fn day3() {
    let f = File::open("inputs/day3.txt").unwrap();
    let r = BufReader::new(f);

    let lines: Vec<String> = r.lines().map(|l| l.unwrap()).collect();

    let n = lines[0].len();
    let m = lines.len();
    let mut field = Field {
        lines: vec![false; n * m],
        n,
        m,
    };

    for (row, line) in lines.iter().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr == '#' {
                field.lines[row * n + col] = true;
            }
        }
    }

    println!("part 1: encountered {} trees", check_slope(&field, 3, 1));

    let part_2_result = check_slope(&field, 1, 1) *
        check_slope(&field, 3, 1) *
        check_slope(&field, 5, 1) *
        check_slope(&field, 7, 1) *
        check_slope(&field, 1, 2);
    println!("part 2: result is {}", part_2_result);
}

fn check_slope(field: &Field, right: usize, down: usize) -> i32 {
    let mut col = 0;
    let mut trees = 0;
    for row in (0..field.m).step_by(down) {
        if field.lines[row * field.n + col] == true {
            trees += 1;
        }
        col = (col + right) % field.n;
    }
    trees
}