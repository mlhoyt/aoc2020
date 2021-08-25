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

    let (ship, waypoint) = instrs.iter().fold(
        (Pos::init(0, 0), Pos::init(10, 1)),
        |(ship, waypoint), instr| translate_ship_and_waypoint(instr, &ship, &waypoint),
    );

    println!(
        "ship={:?} ({}) waypoint={:?}",
        ship,
        (ship.x.abs() + ship.y.abs()),
        waypoint
    );
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

#[derive(Debug, Clone, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn init(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn translate(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn rotate90(&self, dir: &Rotation) -> Self {
        let dx = self.x;
        let dy = self.y;

        match dir {
            Rotation::CCW => Self { x: -1 * dy, y: dx },
            Rotation::CW => Self { x: dy, y: -1 * dx },
        }
    }

    fn rotate90n(&self, dir: &Rotation, n: usize) -> Self {
        (0..n).fold(self.clone(), |pos, _| pos.rotate90(dir))
    }
}

#[derive(Debug)]
enum Rotation {
    CW,
    CCW,
}

#[test]
fn test_pos_rotate90_cw() {
    let pos = Pos::init(3, 2);
    let r90 = pos.rotate90(&Rotation::CW);
    assert_eq!(r90, Pos::init(2, -3));
    let r180 = r90.rotate90(&Rotation::CW);
    assert_eq!(r180, Pos::init(-3, -2));
    let r270 = r180.rotate90(&Rotation::CW);
    assert_eq!(r270, Pos::init(-2, 3));
    let r360 = r270.rotate90(&Rotation::CW);
    assert_eq!(r360, pos);
}

#[test]
fn test_pos_rotate90_ccw() {
    let pos = Pos::init(3, 2);
    let r90 = pos.rotate90(&Rotation::CCW);
    assert_eq!(r90, Pos::init(-2, 3));
    let r180 = r90.rotate90(&Rotation::CCW);
    assert_eq!(r180, Pos::init(-3, -2));
    let r270 = r180.rotate90(&Rotation::CCW);
    assert_eq!(r270, Pos::init(2, -3));
    let r360 = r270.rotate90(&Rotation::CCW);
    assert_eq!(r360, pos);
}

fn translate_ship_and_waypoint(instr: &Instr, ship: &Pos, waypoint: &Pos) -> (Pos, Pos) {
    match instr.dir {
        Direction::N => (ship.clone(), waypoint.translate(0, instr.amount as i32)),
        Direction::E => (ship.clone(), waypoint.translate(instr.amount as i32, 0)),
        Direction::S => (
            ship.clone(),
            waypoint.translate(0, -1 * instr.amount as i32),
        ),
        Direction::W => (
            ship.clone(),
            waypoint.translate(-1 * instr.amount as i32, 0),
        ),
        Direction::F => (
            ship.translate(
                instr.amount as i32 * waypoint.x,
                instr.amount as i32 * waypoint.y,
            ),
            waypoint.clone(),
        ),
        Direction::L => (
            ship.clone(),
            waypoint.rotate90n(&Rotation::CCW, instr.amount / 90),
        ),
        Direction::R => (
            ship.clone(),
            waypoint.rotate90n(&Rotation::CW, instr.amount / 90),
        ),
    }
}
