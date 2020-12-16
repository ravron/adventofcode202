use std::fmt::{Debug, Formatter};
use core::fmt;

pub fn day11() {
    let (p1, p2) = day11_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

#[derive(Clone, Copy, Debug)]
enum Seat {
    Floor,
    Empty,
    Occupied
}

struct Seating {
    active: Vec<Seat>,
    next: Vec<Seat>,
    m: usize,
    n: usize,
}

fn day11_impl() -> (i32, i32) {
    let input = include_str!("../inputs/day11.txt");

    let mut seating = Seating::from_string(input);
    let p1 = seating.simulate();

    let mut seating = Seating::from_string(input);
    let p2 = seating.simulate_sight();

    (p1, p2)
}

impl Seating {
    fn from_string(input: &str) -> Self {
        let m = input.lines().next().unwrap().len();
        let n = input.lines().count();
        let mut active = vec![Seat::Floor; m * n];
        let next = vec![Seat::Floor; m * n];

        for (nn, l) in input.lines().enumerate() {
            for (mm, c) in l.bytes().enumerate() {
                active[nn * m + mm] = match c {
                    b'.' => Seat::Floor,
                    b'L' => Seat::Empty,
                    x => panic!("invalid initial state {}", x),
                };
            }
        }

        Self {
            active,
            next,
            m,
            n,
        }
    }

    fn simulate(&mut self) -> i32 {
        let next_state =
            |seating: &Seating, current: &Seat, mm: usize, nn: usize| {
                let neighbors = seating.neighbor_count(mm, nn);
                match (current, neighbors) {
                    (Seat::Empty, 0) => (Seat::Occupied, true),
                    (Seat::Occupied, 4..=8) => (Seat::Empty, true),
                    (current, _) => (*current, false),
                }
            };
        while self.simulate_one(next_state) {};
        self.active.iter().filter(|c| matches!(c, Seat::Occupied)).count() as i32
    }

    fn simulate_sight(&mut self) -> i32 {
        let next_state =
            |seating: &Seating, current: &Seat, mm: usize, nn: usize| {
                let neighbors = seating.sight_count(mm, nn);
                match (current, neighbors) {
                    (Seat::Empty, 0) => (Seat::Occupied, true),
                    (Seat::Occupied, 5..=8) => (Seat::Empty, true),
                    (current, _) => (*current, false),
                }
            };

        while self.simulate_one(next_state) {};
        self.active.iter().filter(|c| matches!(c, Seat::Occupied)).count() as i32
    }

    fn simulate_one<F>(&mut self, next_state: F) -> bool
        where F: Fn(&Self, &Seat, usize, usize) -> (Seat, bool) {
        let mut any_changed = false;
        for nn in 0..self.n {
            for mm in 0..self.m {
                let current = &self.active[(nn * self.m + mm)];
                let (next, changed) = next_state(self, current, mm, nn);
                self.next[(nn * self.m + mm)] = next;
                if changed { any_changed = true; }
            }
        }

        std::mem::swap(&mut self.active, &mut self.next);

        any_changed
    }

    fn neighbor_count(&self, center_m: usize, center_n: usize) -> usize {
        let mut count: usize = 0;
        let (center_m, center_n) = (center_m as i32, center_n as i32);
        for target_m in center_m -1..=center_m +1 {
            for target_n in center_n -1..=center_n +1 {
                if !self.in_bounds(target_m, target_n) ||
                    (target_m == center_m && target_n == center_n) {
                    continue;
                }
                if let Seat::Occupied = self.active[(target_n * self.m as i32 + target_m) as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    fn sight_count(&self, center_m: usize, center_n: usize) -> usize {
        let deltas: Vec<(i32, i32)> = (-1..=1)
            .map(|dm| (-1..=1).map(move |dn| (dm, dn)))
            .flatten()
            .filter(|d| *d != (0, 0))
            .collect();
        let mut count = 0;
        for (dm, dn) in deltas {
            let (mut cur_m, mut cur_n) = (center_m as i32, center_n as i32);
            let increment = loop {
                cur_m += dm;
                cur_n += dn;
                if !self.in_bounds(cur_m, cur_n) {
                    break 0;
                }
                match self.active[cur_n as usize * self.m + cur_m as usize] {
                    Seat::Empty => break 0,
                    Seat::Occupied => break 1,
                    Seat::Floor => (),
                };
            };
            count += increment;
        }
       count
    }

    fn in_bounds(&self, m: i32, n: i32) -> bool {
        m >= 0 && m < self.m as i32 && n >= 0 && n < self.n as i32
    }
}

impl Debug for Seating {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.active.fmt(f)
    }
}