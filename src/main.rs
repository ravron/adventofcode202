use std::env;
#[macro_use] extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse().expect("enter a number");
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        5 => day5::day5(),
        x => println!("unimplemented day {}", x),
    }
}