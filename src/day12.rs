
pub fn day12() {
    let (p1, p2) = day12_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn day12_impl() -> (i32, i32) {
    let input = include_str!("../inputs/day12.txt");
    let p1 = navigate(input);
    let p2 = navigate_waypoint(input);
    (p1, p2)
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Copy, Clone)]
enum Rotate {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

struct Point {
    x: i32,
    y: i32,
}

impl Instruction {
    fn from_string(s: &str) -> Self {
        let mut chars = s.chars();
        let inst = chars.next().unwrap();
        let num = chars.collect::<String>().parse::<i32>().unwrap();
        match inst {
            'N' => Instruction::North(num),
            'E' => Instruction::East(num),
            'S' => Instruction::South(num),
            'W' => Instruction::West(num),
            'L' => Instruction::Left(num),
            'R' => Instruction::Right(num),
            'F' => Instruction::Forward(num),
            x => panic!("invalid instruction char {}", x),
        }
    }
}

impl Heading {
    fn adjust(self, direction: Rotate, amount: i32) -> Self {
        let cur = self as i32;
        let delta = match (direction, amount) {
            (_, 0) => 0,
            (_, 180) => 2,
            (Rotate::Right, 90) | (Rotate::Left, 270) => 1,
            (Rotate::Right, 270) | (Rotate::Left, 90) => 3,
            (_, x) => panic!("invalid rotation amount {}", x),
        };
        match (cur + delta) % 4 {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            x => panic!("got invalid result {}", x),
        }
    }
}

impl Point {
    fn rotate(self, direction: Rotate, amount: i32) -> Self {
        match (direction, amount) {
            (_, 0) => self,
            (_, 180) => Point { x: -self.x, y: -self.y },
            (Rotate::Right, 90) | (Rotate::Left, 270) =>
                Point { x: self.y, y: -self.x },
            (Rotate::Right, 270) | (Rotate::Left, 90) =>
                Point { x: -self.y, y: self.x },
            (_, x) => panic!("invalid rotation amount {}", x),
        }
    }
}


fn parse_instructions(input: &str) -> impl Iterator<Item=Instruction> + '_ {
    input.lines().map(Instruction::from_string)
}

fn navigate(input: &str) -> i32 {
    let mut heading = Heading::East;
    let mut ship = Point { x: 0, y: 0 };
    for inst in parse_instructions(input) {
        match inst {
            Instruction::North(delta) => ship.y += delta,
            Instruction::East(delta) => ship.x += delta,
            Instruction::South(delta) => ship.y -= delta,
            Instruction::West(delta) => ship.x -= delta,
            Instruction::Left(delta) =>
                heading = heading.adjust(Rotate::Left, delta),
            Instruction::Right(delta) =>
                heading = heading.adjust(Rotate::Right, delta),
            Instruction::Forward(delta) => match heading {
                Heading::North => ship.y += delta,
                Heading::East => ship.x += delta,
                Heading::South => ship.y -= delta,
                Heading::West => ship.x -= delta,
            }
        }
    }
    ship.x.abs() + ship.y.abs()
}

fn navigate_waypoint(input: &str) -> i32 {
    let mut ship = Point { x: 0, y: 0 };
    let mut way = Point { x: 10, y: 1 };
    for inst in parse_instructions(input) {
        match inst {
            Instruction::North(delta) => way.y += delta,
            Instruction::East(delta) => way.x += delta,
            Instruction::South(delta) => way.y -= delta,
            Instruction::West(delta) => way.x -= delta,
            Instruction::Left(delta) =>
                way = way.rotate(Rotate::Left, delta),
            Instruction::Right(delta) =>
                way = way.rotate(Rotate::Right, delta),
            Instruction::Forward(delta) => {
                ship.x += way.x * delta;
                ship.y += way.y * delta;
            }
        }
    }
    ship.x.abs() + ship.y.abs()
}

