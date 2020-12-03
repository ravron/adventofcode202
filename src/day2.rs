use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::RangeInclusive;

pub fn day2() {
    let f = File::open("day2.txt").unwrap();
    let buf_reader = BufReader::new(f);

    let (mut valid_pt_1_count, mut valid_pt_2_count) = (0, 0);

    for line in buf_reader.lines().map(|l| l.unwrap()) {
        let splits: Vec<&str> = line.split_whitespace().collect();
        let count_range: RangeInclusive<usize> = {
            let ends: Vec<&str> = splits[0].split('-').collect();
            ends[0].parse().unwrap()..=ends[1].parse().unwrap()
        };
        let letter = splits[1].chars().next().unwrap();
        let password = splits[2];

        if is_valid_part_1(&count_range, letter, password) {
            valid_pt_1_count += 1;
        }

        if is_valid_part_2(&count_range, letter, password) {
            valid_pt_2_count += 1;
        }
    }

    println!("part 1: {} valid passwords", valid_pt_1_count);
    println!("part 2: {} valid passwords", valid_pt_2_count);
}

fn is_valid_part_1(count_range: &RangeInclusive<usize>, letter: char, password: &str) -> bool {
    count_range.contains(&password.matches(letter).count())
}

fn is_valid_part_2(letter_range: &RangeInclusive<usize>, letter: char, password: &str) -> bool {
    (password.chars().nth(*letter_range.start()-1).unwrap() == letter) !=
        (password.chars().nth(*letter_range.end()-1).unwrap() == letter)
}
