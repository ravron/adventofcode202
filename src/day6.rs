use std::collections::HashMap;

struct Group {
    size: usize,
    answers: HashMap<char, usize>,
}

pub fn day6() {
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
    println!("part 1: {}", part_1_sum);

    let part_2_sum: usize = groups
        .iter()
        .map(|g|
            g.answers.values().filter(|v| **v == g.size).count()
        )
        .sum();
    println!("part 2: {}", part_2_sum);
}