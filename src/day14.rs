use std::collections::HashMap;
use regex::Regex;

pub fn day14() {
    let (p1, p2) = day14_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

struct Mask {
    set: usize,
    clear: usize,
    floating: Vec<usize>,
}

impl Mask {
    fn from_string(s: &str) -> Self {
        let (mut set, mut clear): (usize, usize) = (0, 0);
        let mut floating: Vec<usize> = Vec::with_capacity(36);
        for (i, b) in s.bytes().enumerate() {
            match b {
                b'1' => set |= 1 << (35 - i),
                b'0' => clear |= 1 << (35 - i),
                b'X' => floating.push(35 - i),
                x => panic!("invalid mask char {}", x),
            }
        }
        Mask { set, clear, floating }
    }

    fn modify_value(&self, val: usize) -> usize {
        val & !self.clear | self.set
    }

    fn explode_addr(&self, mut addr: usize) -> Vec<usize> {
        // Step 1: set all the bits in addr identified by set
        addr |= self.set;

        // Step 2: explode out a list of permutations of floating bits
        // There will be 2^n such permutations, where n is the number of
        // floating bits. Prefill the exploded vec with the un-mutated addr.
        let mut exploded: Vec<usize> = vec![addr; 1 << self.floating.len()];
        for (i, ex) in exploded.iter_mut().enumerate() {
            for (j, float) in self.floating.iter().enumerate() {
                if i & (1 << j) != 0 {
                    // Bit float in addr should be set
                    *ex |= 1 << float;
                } else {
                    // Bit float in addr should be cleared
                    *ex &= !(1 << float);
                }
            }
        }
        exploded
    }
}

fn day14_impl() -> (usize, usize) {
    let input = include_str!("../inputs/day14.txt");

    lazy_static! {
        static ref RE: regex::Regex = Regex::new(r"(?:mask|mem\[(\d+)]) = (\w+)").unwrap();
    }

    let mut mem1: HashMap<usize, usize> = HashMap::new();
    let mut mem2: HashMap<usize, usize> = HashMap::new();
    let mut mask = Mask{ set: 0, clear: 0, floating: vec![] };

    for line in input.lines() {
        let captures: regex::Captures = RE.captures(line).unwrap();
        match (captures.get(1), captures.get(2)) {
            (None, Some(mask_m)) =>
                mask = Mask::from_string(mask_m.as_str()),
            (Some(addr_m), Some(val_m)) => {
                let val = val_m.as_str().parse().unwrap();
                let target_addr = addr_m.as_str().parse().unwrap();

                // p1
                mem1.insert(target_addr, mask.modify_value(val));

                // p2
                for addr in mask.explode_addr(target_addr) {
                    mem2.insert(addr, val);
                }
            },
            (x, y) =>
                panic!("invalid matches {:?}, {:?}", x, y),
        }
    }

    let p1 = mem1.values().sum();
    let p2 = mem2.values().sum();
    (p1, p2)
}