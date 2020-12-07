pub fn day5() {
    let (p1, p2) = day5_impl();
    println!("part 1: max seat is {}", p1);
    println!("part 2: your seat is {}", p2);
}

fn day5_impl() -> (usize, usize) {
    let input = include_str!("../inputs/day5.txt");

    let mut all_seats = [false; 1024];

    let mut min_seat: u16 = 1024;
    let mut max_seat: u16 = 0;
    for seat in input.lines().map(parse_seat) {
        all_seats[seat as usize] = true;
        min_seat = min_seat.min(seat);
        max_seat = max_seat.max(seat);
    }

    let mut all_seats: Vec<u16> = input.lines()
        .map(parse_seat)
        .collect();

    all_seats.sort();

    let max_seat = *all_seats.last().unwrap();

    let mut part_2_seat: u16 = 0;

    let first_seat = all_seats[0];
    for (i, seat) in all_seats.iter().enumerate() {
        if i + first_seat as usize != *seat as usize {
            // Minus 1 because when we detect the discrepancy, we've passed
            // your seat.
            part_2_seat = seat - 1;
            break;
        }
    }
    (max_seat as usize, part_2_seat as usize)
}

fn parse_seat(seat: &str) -> u16 {
    let mut seat_id: u16 = 0;
    for (i, c) in seat.bytes().enumerate() {
        match c {
            b'B' | b'R' => seat_id |= 0x200 >> i,
            b'F' | b'L' => (),
            _ => panic!("invalid char {}", c),
        }
    }
    seat_id
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_day5() {
        let (p1, p2) = day5_impl();
        assert_eq!(p1, 850);
        assert_eq!(p2, 599);
    }

    #[bench]
    fn bench_day5(b: &mut Bencher) {
        b.iter(|| day5_impl());
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| parse_seat("BFFBFBFLRL"));
    }
}