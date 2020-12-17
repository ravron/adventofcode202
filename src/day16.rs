use std::ops::RangeInclusive;

pub fn day16() {
    let (p1, p2) = day16_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn day16_impl() -> (usize, usize) {
    let input = include_str!("../inputs/day16.txt");

    let mut sections = input.split("\n\n");
    let mut fields: Vec<Field> = vec![];

    for line in sections.next().unwrap().lines() {
        fields.push(Field::from_string(line));
    }


    let my_ticket: Vec<usize> = sections.next().unwrap().lines()
        .skip(1).next().unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap()).collect();


    // p1
    // valid_position_samples is the column-major ticket and position matrix.
    // In other words, valid_position_samples[N] contains every value found in
    // position N on each valid ticket.
    let mut valid_position_samples: Vec<Vec<usize>> = vec![vec![]; fields.len()];
    let mut error_rate: usize = 0;
    for line in sections.next().unwrap().lines().skip(1) {
        let ticket: Vec<_> = line.split(',')
            .map(|s| s.parse::<usize>().unwrap()).collect();

        let bad_fields: Vec<_> = ticket
            .iter()
            .filter(|n| !fields.iter().any(|f| f.contains(**n)))
            .map(|n| *n)
            .collect();

        error_rate += bad_fields.iter().sum::<usize>();
        if bad_fields.len() == 0 {
            for (i, n) in ticket.iter().enumerate() {
                valid_position_samples[i].push(*n);
            }
        }
    }

    // p2
    // After the loop, position_fields is full of Some(Field), where the index
    // in the vec indicates which position corresponds with the Field at that
    // index.
    let mut position_fields: Vec<Option<Field>> = vec![None; fields.len()];
    while !fields.is_empty() {
        for (position, samples) in valid_position_samples.iter().enumerate() {
            let possible_fields = fields
                .iter()
                .enumerate()
                // Find every field that could be in this position
                .filter_map(|(j, f)|
                    if samples.iter().all(|s| f.contains(*s)) {
                        Some(j)
                    } else {
                        None
                    })
                .collect::<Vec<_>>();
            // If there is exactly one field that could be in this position,
            // lock it in by moving it from fields to position_fields
            if possible_fields.len() != 1 { continue }
            position_fields[position] = Some(fields.remove(possible_fields[0]))
        }
    }

    // Flatten optionals
    let position_fields = position_fields
        .into_iter()
        .map(|f| f.unwrap())
        .collect::<Vec<_>>();
    let mut depature_vals: Vec<usize> = vec![];
    for (position, field) in position_fields.iter().enumerate() {
        if !field.name.starts_with("departure") {
            continue
        }
        depature_vals.push(my_ticket[position]);
    }
    let p2 = depature_vals.iter().product::<usize>();


    (error_rate, p2)
}

#[derive(Debug, Clone)]
struct Field<'a> {
    name: &'a str,
    r1: RangeInclusive<usize>,
    r2: RangeInclusive<usize>,
}


impl<'a> Field<'a> {
    fn from_string(input: &'a str) -> Self {
        lazy_static! {
            static ref RANGE_RE: regex::Regex = regex::Regex::new(r"(?P<start>\d+)-(?P<end>\d+)").unwrap();
        }

        let name = input.split(':').next().unwrap();

        let mut capture_matches: regex::CaptureMatches = RANGE_RE.captures_iter(input);
        let mut get_range = || {
            let c = capture_matches.next().unwrap();
            c.name("start").unwrap().as_str().parse::<usize>().unwrap()..=
                c.name("end").unwrap().as_str().parse::<usize>().unwrap()
        };

        Field { name, r1: get_range(), r2: get_range() }
    }

    fn contains(&self, val: usize) -> bool {
        self.r1.contains(&val) || self.r2.contains(&val)
    }
}