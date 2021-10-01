use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

// const FILE_NAME: &str = "input/day17.test.txt";
const FILE_NAME: &str = "input/day17.txt";

fn main() {
    let file = fs::File::open(FILE_NAME).expect("cannot open file");
    let mut file = BufReader::new(file);
    let mut input = String::new();
    while file.read_line(&mut input).unwrap() > 0 {}
    println!("{}", input);

    let seed: Seed = parse(&input).expect("failed to parse input data");
    println!("{:?}", seed);

    let mut cube = Cube::new(&seed);
    for _ in 0..6 {
        cube = cube.next_state();
    }

    println!("{}", cube.num_active_cells());
}

type Seed = Vec<(isize, isize)>;

enum CellState {
    Inactive,
    Active,
}

fn parse(s: &str) -> Result<Seed, parser::ParseErr> {
    parser::parse(s)
}

mod parser {
    use crate::CellState;
    use crate::Seed;

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Seed, ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Seed
                = rs:row() ++ eol()
                {
                    rs.iter().enumerate().map(|(r, vs)| {
                        vs.iter().enumerate().map(move |(c, state)| match state {
                            CellState::Inactive => None,
                            CellState::Active => Some((r as isize, c as isize)),
                        })
                    })
                    .flatten()
                    .filter_map(|v| v)
                    .collect::<Vec<_>>()
                }

            rule row() -> Vec<CellState>
                = vs:cell_state()+ { vs }

            rule cell_state() -> CellState
                = "." { CellState::Inactive }
                / "#" { CellState::Active }

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn get_neighbors(&self) -> Vec<Self> {
        let mut ps = Vec::new();

        for xp in self.x - 1..=self.x + 1 {
            for yp in self.y - 1..=self.y + 1 {
                for zp in self.z - 1..=self.z + 1 {
                    if !(xp == self.x && yp == self.y && zp == self.z) {
                        ps.push(Self::new(xp, yp, zp));
                    }
                }
            }
        }

        ps
    }
}

struct Cube {
    state: HashSet<Point>,
}

impl Cube {
    fn new(seed: &Seed) -> Self {
        let mut state = HashSet::new();
        seed.iter().for_each(|(x, y)| {
            state.insert(Point::new(*x, *y, 0));
        });

        Self { state }
    }

    fn next_state(&self) -> Self {
        let mut next = Self {
            state: HashSet::new(),
        };

        self.get_all_points().into_iter().for_each(|pt| {
            let num_active_neighbors = pt
                .get_neighbors()
                .into_iter()
                .filter(|v| self.is_active(v))
                .count();

            if self.is_active(&pt) {
                if num_active_neighbors == 2 || num_active_neighbors == 3 {
                    next.set_active(&pt);
                }
            } else {
                if num_active_neighbors == 3 {
                    next.set_active(&pt);
                }
            }
        });

        next
    }

    fn get_all_points(&self) -> Vec<Point> {
        if self.state.len() == 0 {
            return Vec::new();
        }

        let mut coords = self.state.iter();
        let first_coord = coords.next().unwrap();
        let sr = (
            (first_coord.x, first_coord.x),
            (first_coord.y, first_coord.y),
            (first_coord.z, first_coord.z),
        );

        let ((x_min, x_max), (y_min, y_max), (z_min, z_max)) =
            coords.fold(sr, |(xr, yr, zr), pt| {
                (
                    (xr.0.min(pt.x), xr.1.max(pt.x)),
                    (yr.0.min(pt.y), yr.1.max(pt.y)),
                    (zr.0.min(pt.z), zr.1.max(pt.z)),
                )
            });

        let mut ps = Vec::new();

        for x in (x_min - 1..=x_max + 1).to_owned() {
            for y in (y_min - 1..=y_max + 1).to_owned() {
                for z in (z_min - 1..=z_max + 1).to_owned() {
                    ps.push(Point::new(x, y, z));
                }
            }
        }

        ps
    }

    fn is_active(&self, point: &Point) -> bool {
        self.state.contains(point)
    }

    fn set_active(&mut self, point: &Point) {
        self.state.insert(point.clone());
    }

    fn num_active_cells(&self) -> usize {
        self.state.len()
    }
}
