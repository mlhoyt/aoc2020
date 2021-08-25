use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    // let input = include_str!("../../input/day12.test.txt");
    let input = include_str!("../../input/day12.txt");

    let instrs = input
        .lines()
        .map(|v| v.parse::<Instr>().unwrap_or_else(|_| panic!()))
        .collect::<Vec<_>>();

    let mut pos = Pos::new();
    instrs.iter().for_each(|instr| pos.execute(instr));

    println!("x={} y={} md={}", pos.x, pos.y, (pos.x.abs() + pos.y.abs()));
}

#[derive(Debug)]
struct Instr {
    dir: Direction,
    amount: usize,
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

impl FromStr for Instr {
    type Err = InstrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cs = s.chars();
        let dir_str = cs.clone().into_iter().take(1).collect::<String>();
        let amount_str = cs.into_iter().skip(1).collect::<String>();

        let amount = amount_str.parse::<usize>()?;

        let dir = match dir_str.as_str() {
            "N" => Direction::N,
            "S" => Direction::S,
            "E" => Direction::E,
            "W" => Direction::W,
            "L" => Direction::L,
            "R" => Direction::R,
            "F" => Direction::F,
            _ => return Err(InstrParseError::new("unrecognized direction value")),
        };

        Ok(Instr { dir, amount })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InstrParseError {
    details: String,
}

impl InstrParseError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InstrParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.details)
    }
}

impl Error for InstrParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for InstrParseError {
    fn from(err: ParseIntError) -> Self {
        #[allow(deprecated)]
        InstrParseError::new(err.description())
    }
}

#[derive(Debug)]
struct Pos {
    heading: Heading,
    x: i32,
    y: i32,
}

impl Pos {
    fn new() -> Self {
        Self {
            heading: Heading::E,
            x: 0,
            y: 0,
        }
    }

    fn execute(&mut self, instr: &Instr) {
        match instr.dir {
            Direction::N => self.y += instr.amount as i32,
            Direction::S => self.y -= instr.amount as i32,
            Direction::E => self.x += instr.amount as i32,
            Direction::W => self.x -= instr.amount as i32,
            Direction::F => match self.heading {
                Heading::N => self.y += instr.amount as i32,
                Heading::S => self.y -= instr.amount as i32,
                Heading::E => self.x += instr.amount as i32,
                Heading::W => self.x -= instr.amount as i32,
            },
            Direction::L => self.heading = self.heading.clone().turn(Rotation::CCW, instr.amount),
            Direction::R => self.heading = self.heading.clone().turn(Rotation::CW, instr.amount),
        };
    }
}

#[derive(Debug, Clone)]
enum Heading {
    N = 90,
    E = 0,
    W = 180,
    S = 270,
}

#[derive(Debug)]
enum Rotation {
    CW,
    CCW,
}

impl Heading {
    fn turn(self, dir: Rotation, amount: usize) -> Self {
        let mut degrees = match dir {
            Rotation::CW => ((self as isize) - (amount as isize)) % 360,
            Rotation::CCW => ((self as isize) + (amount as isize)) % 360,
        };

        if degrees < 0 {
            degrees = 360 + degrees;
        }

        Self::from_degrees(degrees)
    }

    fn from_degrees(v: isize) -> Self {
        if v >= 0 && v < 90 {
            Self::E
        } else if v >= 90 && v < 180 {
            Self::N
        } else if v >= 180 && v < 270 {
            Self::W
        } else {
            Self::S
        }
    }
}
