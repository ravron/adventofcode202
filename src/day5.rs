pub fn day5() {
    let (p1, p2) = day5_impl();
    println!("part 1: max seat is {}", p1);
    println!("part 2: your seat is {}", p2);
}

fn day5_impl() -> (usize, usize) {
    let input = include_str!("../inputs/day5.txt");

    let mut all_seats = [false; 1024];

    let mut min_seat: usize = u16::MAX as usize;
    let mut max_seat: usize = u16::MIN as usize;
    for seat in input.lines().map(parse_seat) {
        all_seats[seat] = true;
        min_seat = if seat < min_seat { seat } else { min_seat };
        max_seat = if seat > max_seat { seat } else { max_seat };
    }

    let mut part_2_seat: usize = 0;

    for (i, seat) in all_seats[min_seat..=max_seat].iter().enumerate() {
        if !seat {
            part_2_seat = i + min_seat;
            break;
        }
    }
    (max_seat, part_2_seat)
}

fn parse_seat(seat: &str) -> usize {
    let mut seat_id: usize = 0;
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