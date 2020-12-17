use std::collections::HashMap;

pub fn day15() {
    let (p1, p2) = day15_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn day15_impl() -> (usize, usize) {
    const STARTING: [usize; 6] = [0, 13, 1, 8, 6, 15];
    const P1_TARGET_TURN: usize = 2020;
    const P2_TARGET_TURN: usize = 30000000;

    let mut memo: Vec<Option<usize>> = vec![None; P2_TARGET_TURN];

    for (turn, number) in STARTING.iter().enumerate()
        .take(STARTING.len() - 1) {
        let turn = turn + 1;
        memo[*number] = Some(turn);
    }

    let mut last_number = *STARTING.last().unwrap();
    let mut p1: usize = 0;
    for turn in STARTING.len() + 1..=P2_TARGET_TURN {
        last_number = next_number(turn, last_number, &mut memo);
        if turn == P1_TARGET_TURN {
            p1 = last_number;
        }
    }
    (p1, last_number)
}

fn next_number(turn: usize, last_number: usize, memo: &mut Vec<Option<usize>>) -> usize {
    let result: usize;
    if let Some(last_spoken_turn) = memo[last_number] {
        result = turn - 1 - last_spoken_turn;
    } else {
        result = 0;
    }
    memo[last_number] = Some(turn - 1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_day15 () {
        let (p1, p2) = day15_impl();
        assert_eq!(p1, 1618);
        assert_eq!(p2, 548531);
    }

    #[bench]
    #[ignore]
    // Too slow…
    fn bench_day15(b: &mut Bencher) {
        b.iter(day15_impl);
    }
}
