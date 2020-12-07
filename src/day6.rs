use std::collections::HashMap;

struct Group {
    size: usize,
    answers: HashMap<char, usize>,
}

pub fn day6() {
    let (p1, p2) = day6_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn day6_impl() -> (usize, usize) {
    let input = include_str!("../inputs/day6.txt");

    let mut groups: Vec<Group> = Vec::new();
    for group_lines in input.split("\n\n") {
        let mut group = Group { size: 0, answers: Default::default() };
        for member_line in group_lines.split_whitespace() {
            group.size += 1;
            for answer in member_line.chars() {
                *group.answers.entry(answer).or_default() += 1;
            }
        }
        groups.push(group);
    }

    let part_1_sum: usize = groups
        .iter()
        .map(|g| g.answers.keys().len())
        .sum();

    let part_2_sum: usize = groups
        .iter()
        .map(|g|
            g.answers.values().filter(|v| **v == g.size).count()
        )
        .sum();
    (part_1_sum, part_2_sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_day6() {
        let (p1, p2) = day6_impl();
        assert_eq!(p1, 6763);
        assert_eq!(p2, 3512);
    }

    #[bench]
    fn bench_day6(b: &mut Bencher) {
        b.iter(|| day6_impl());
    }
}