use std::env;

mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse().expect("enter a number");
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        x => println!("unimplemented day {}", x),
    }
}