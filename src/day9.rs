use itertools::Itertools;

pub fn day9() {
    let (p1, p2) = day9_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn day9_impl() -> (i64, i64) {
    let input = include_str!("../inputs/day9.txt");

    const PREAMBLE_LEN: usize = 25;

    let data: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut p1: i64 = 0;
    for i in PREAMBLE_LEN..data.len() {
        if !check_pair_sums_to(&data[i-PREAMBLE_LEN..i], data[i]) {
            p1 = data[i];
            break;
        }
    }

    let mut p2: i64 = 0;
    'outer: for chunk_size in 2..=data.len() {
        for i in 0..=data.len()-chunk_size {
            let chunk = &data[i..i+chunk_size];
            if chunk.iter().sum::<i64>() == p1 {
                p2 = chunk.iter().min().unwrap() + chunk.iter().max().unwrap();
                break 'outer;
            }
        }
    }

    (p1, p2)
}

fn check_pair_sums_to(range: &[i64], target: i64) -> bool {
    range.iter().combinations(2).any(|v| v[0] + v[1] == target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_day9 () {
        let (p1, p2) = day9_impl();
        assert_eq!(p1, 41682220);
        assert_eq!(p2, 5388976);
    }

    #[bench]
    fn bench_day9(b: &mut Bencher) {
        b.iter(day9_impl);
    }
}
