pub fn day5() {
    let input = include_str!("../inputs/day5.txt");

    let max_seat = input.lines()
        .map(parse_seat)
        .max_by(|a, b| a.cmp(b))
        .unwrap();

    println!("part 1: max seat is {}", max_seat);

    let mut all_seats = input.lines()
        .map(parse_seat)
        .collect::<Vec<u16>>();

    all_seats.sort();

    let first_seat = all_seats[0];
    for (i, seat) in all_seats.iter().enumerate() {
        if i + first_seat as usize != *seat as usize {
            // Minus 1 because when we detect the discrepancy, we've passed
            // your seat.
            println!("part 2: your seat is {}", seat-1);
            break;
        }
    }
}

fn parse_seat(seat: &str) -> u16 {
    let mut seat_id: u16 = 0;
    for (i, c) in seat.chars().enumerate() {
        match c {
            'B' | 'R' => seat_id |= 0x200 >> i,
            'F' | 'L' => (),
            _ => panic!("invalid row {}", c),
        };
    }
    seat_id
}
