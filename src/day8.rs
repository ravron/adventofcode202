pub fn day8() {
    let (p1, p2) = day8_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

struct Prog {
    insts: Vec<Inst>,
}

#[derive(Copy, Clone)]
enum Inst {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

enum Result {
    Looped(i32),
    Terminated(i32),
}

fn day8_impl() -> (i32, i32) {
    let input = include_str!("../inputs/day8.txt");
    let mut prog = Prog::from_input(input);
    println!("{} insts", prog.insts.len());

    let p1 = prog.acc_at_loop();
    let p2 = prog.acc_after_fix();
    (p1, p2)
}

impl Prog {
    fn from_input(input: &str) -> Self {
        let mut insts: Vec<Inst> = vec![];
        for line in input.lines() {
            let words: Vec<&str> = line.split_ascii_whitespace().collect();
            insts.push(match words[0] {
                "nop" => Inst::Nop(words[1].parse().unwrap()),
                "acc" => Inst::Acc(words[1].parse().unwrap()),
                "jmp" => Inst::Jmp(words[1].parse().unwrap()),
                x => panic!("invalid inst {}", x),
            })
        }
        Prog{ insts }
    }

    fn acc_at_loop(&self) -> i32 {
        if let Result::Looped(acc) = self.run() {
            return acc;
        }
        panic!("expected loop");
    }

    fn acc_after_fix(&mut self) -> i32 {
        for i in 0..self.insts.len() {
            let inst = self.insts[i];
            match inst {
                Inst::Nop(inc) => {
                    self.insts[i] = Inst::Jmp(inc);
                    if let Result::Terminated(acc) = self.run() {
                        return acc;
                    }
                    self.insts[i] = inst;
                },
                Inst::Jmp(inc) => {
                    self.insts[i] = Inst::Nop(inc);
                    if let Result::Terminated(acc) = self.run() {
                        return acc;
                    }
                    self.insts[i] = inst;
                }
                Inst::Acc(_) => continue,
            }
        }
        panic!("no single patch fixed the program");
    }

    fn run(&self) -> Result {
        let mut executed = vec![false; self.insts.len()];
        let mut pc: usize = 0;
        let mut acc: i32 = 0;
        loop {
            if pc >= self.insts.len() {
                return Result::Terminated(acc);
            }

            if executed[pc] {
                return Result::Looped(acc);
            }
            executed[pc] = true;

            match self.insts[pc] {
                Inst::Nop(_) => pc += 1,
                Inst::Acc(inc) => {
                    acc += inc;
                    pc += 1;
                },
                Inst::Jmp(inc) => pc = (pc as i32 + inc) as usize,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8() {
        let (p1, p2) = day8_impl();
        assert_eq!(p1, 1200);
        assert_eq!(p2, 1023);
    }
}