
pub fn day10() {
    let (p1, p2) = day10_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn day10_impl() -> (u64, u64) {
    let input = include_str!("../inputs/day10.txt");

    let adapters = {
        // 0 for the seat
        let mut a = vec![0 as u64];
        a.append(&mut input.lines().map(|l| l.parse().unwrap()).collect());
        a.sort();
        // largest + 3 for the built-in adapter
        a.push(*a.last().unwrap() + 3);
        a
    };

    let (mut ones, mut threes): (u64, u64) = (0, 0);
    for i in 1..adapters.len() {
        match adapters[i] - adapters[i-1] {
            1 => ones += 1,
            3 => threes += 1,
            2 => (),
            x => panic!("invalid spacing {}", x),
        }
    }
    let p1 = ones * threes;

    let p2 = ways_from(0, &adapters);

    (p1, p2)
}

fn ways_from(i: usize, adapters: &Vec<u64>) -> u64 {
    let mut memo = vec![0 as u64; adapters.len()];
    *memo.last_mut().unwrap() = 1;
    ways_from_impl(i, adapters, &mut memo)
}

fn ways_from_impl(i: usize, adapters: &Vec<u64>, memo: &mut Vec<u64>) -> u64 {
    if memo[i] != 0 {
        return memo[i];
    }

    let next_indices = (i+1..i+4)
        .filter(|j| *j < adapters.len() && adapters[*j] - adapters[i] <= 3);
    let ways: u64 = next_indices
        .map(|j| ways_from_impl(j, adapters, memo))
        .sum();
    memo[i] = ways;
    ways
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_day10 () {
        let (p1, p2) = day10_impl();
        assert_eq!(p1, 2346);
        assert_eq!(p2, 6044831973376);
    }

    #[bench]
    fn bench_day9(b: &mut Bencher) {
        b.iter(day10_impl);
    }
}
