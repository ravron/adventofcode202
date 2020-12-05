pub fn day5() {
    let input = include_str!("../inputs/day5.txt");

    let max_seat = input.lines()
        .map(parse_seat)
        .map(to_seat_id)
        .max_by(|a, b| a.cmp(b))
        .unwrap();

    println!("part 1: max seat is {}", max_seat);

    let mut all_seats = input.lines()
        .map(parse_seat)
        .map(to_seat_id)
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

fn parse_seat(seat: &str) -> (u8, u8) {
    let mut row: u8 = 0;
    let mut mask: u8 = 0x40;
    for c in seat.chars().take(7) {
        match c {
            'F' => row &= !mask,
            'B' => row |= mask,
            _ => panic!("invalid row {}", c),
        };
        mask >>= 1;
    }

    let mut col: u8 = 0;
    mask = 0x4;
    for c in seat.chars().skip(7).take(3) {
        match c {
            'L' => col &= !mask,
            'R' => col |= mask,
            _ => panic!("invalid column {}", c),
        };
        mask >>= 1;
    }

    (row, col)
}

fn to_seat_id((row, col): (u8, u8)) -> u16 {
    row as u16 * 8 + col as u16
}