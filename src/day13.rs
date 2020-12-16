pub fn day13() {
    let (p1, p2) = day13_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn day13_impl() -> (usize, usize) {
    let input = include_str!("../inputs/day13.txt");
    let mut lines = input.lines();
    let depart_line = lines.next().unwrap();
    let buses_line = lines.next().unwrap();

    let depart_ts = depart_line.parse::<usize>().unwrap();
    let buses: Vec<usize> = buses_line
        .split(',')
        .filter_map(|b| b.parse::<usize>().ok())
        .collect();
    println!("ts: {}, buses: {:?}", depart_ts, buses);

    let (delay, best_bus) = buses.iter()
        .map(|b| (b - (depart_ts % b), b))
        .min().unwrap();
    let p1 = delay * best_bus;

    // p2
    let (offset, buses): (Vec<usize>, Vec<usize>) = buses_line
        .split(',')
        .enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(i, b)| (i, b.parse::<usize>().unwrap()))
        .unzip();

    let mut t = buses[0];
    let mut total_iters: usize = 0;
    for i in 2..=buses.len() {
        let r = next_matching_ts(&buses[0..i], &offset[0..i], t);
        t = r.0;
        total_iters += r.1;
    }
    println!("{} total iters", total_iters);

    (p1, t)
}

fn next_matching_ts(buses: &[usize], offsets: &[usize], start: usize) -> (usize, usize) {
    let max = buses.iter().product();
    // product of 0..n-1 is step
    let step = buses.iter().take(buses.len() - 1).product();
    let mut iters: usize = 0;
    for i in (start..max).step_by(step) {
        iters += 1;
        if (i + offsets.last().unwrap()) % buses.last().unwrap() == 0 {
            return (i, iters);
        }
    }
    panic!("couldn't find ts after iterating to {}", max);
}