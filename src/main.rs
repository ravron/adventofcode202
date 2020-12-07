// This nonsense is required to have benchmarking coexist while allowing
// the normal program to run on stable.
#![cfg_attr(feature = "unstable", feature(test))]

use std::env;
#[macro_use] extern crate lazy_static;

#[cfg(feature = "unstable")]
extern crate test;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse().expect("enter a number");
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        5 => day5::day5(),
        6 => day6::day6(),
        x => println!("unimplemented day {}", x),
    }
}