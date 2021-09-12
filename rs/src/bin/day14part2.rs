use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

// const FILE_NAME: &str = "input/day14.test.txt";
// const FILE_NAME: &str = "input/day14.test2.txt";
const FILE_NAME: &str = "input/day14.txt";

pub fn main() {
    let file = File::open(FILE_NAME).expect("Cannot open file.");
    let file = BufReader::new(file);

    let instrs: Vec<_> = file
        .lines()
        .map(|l| l.expect("Cannot read line."))
        .map(|s| s.parse::<Instr>().expect("Failed to parse instruction."))
        .collect();

    let mut mask = Mask::new();

    let result = instrs
        .into_iter()
        .map(|v| match v {
            Instr::Mask(m) => {
                mask = Mask(m);
                None
            }
            Instr::Assignment(a, v) => Some(
                (mask.clone() | a.into())
                    .to_combinations()
                    .into_iter()
                    .filter_map(|ma| ma.to_usize().ok())
                    .map(|ma| (ma, v))
                    .collect::<Vec<_>>(),
            ),
        })
        .filter_map(|v| v)
        .flatten()
        .collect::<HashMap<_, _>>()
        .iter()
        .fold(0, |acc, (_, v)| acc + v);
    println!("{:?}", result);
}

#[derive(Debug, PartialEq, Clone)]
struct Mask(Vec<BitValue>);

#[derive(Debug)]
enum MaskErr {
    ToUsize,
}

impl fmt::Display for MaskErr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "unable to convert Mask to usize")
    }
}

impl Error for MaskErr {
    fn description(&self) -> &str {
        "unable to convert Mask to usize"
    }
}

impl From<ParseIntError> for MaskErr {
    fn from(_: ParseIntError) -> Self {
        Self::ToUsize
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self(std::iter::repeat(BitValue::V0).take(36).collect::<Vec<_>>())
    }
}

impl Mask {
    fn new() -> Self {
        Default::default()
    }

    fn to_combinations(&self) -> Vec<Self> {
        let mut results = Vec::new();

        if self.0.contains(&BitValue::VX) {
            let mut i = 0;
            while i < self.0.len() {
                if self.0[i] == BitValue::VX {
                    let mut v0 = self.clone();
                    v0.0[i] = BitValue::V0;
                    v0.to_combinations()
                        .into_iter()
                        .for_each(|v| results.push(v));

                    let mut v1 = self.clone();
                    v1.0[i] = BitValue::V1;
                    v1.to_combinations()
                        .into_iter()
                        .for_each(|v| results.push(v));

                    break;
                }

                i += 1;
            }
        } else {
            results.push(self.clone())
        }

        results
    }

    fn to_usize(&self) -> Result<usize, MaskErr> {
        let mask_str = self
            .0
            .iter()
            .map(|v| v.to_char())
            .filter_map(|v| v)
            .collect::<String>();

        match usize::from_str_radix(&mask_str, 2) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

#[test]
fn test_mask_to_combinations() {
    let mask = Mask(
        vec![
            std::iter::repeat(BitValue::V0).take(32).collect::<Vec<_>>(),
            vec![BitValue::V0, BitValue::VX, BitValue::V1, BitValue::VX],
        ]
        .concat(),
    );
    let expected = vec![
        Mask(
            vec![
                std::iter::repeat(BitValue::V0).take(32).collect::<Vec<_>>(),
                vec![BitValue::V0, BitValue::V0, BitValue::V1, BitValue::V0],
            ]
            .concat(),
        ),
        Mask(
            vec![
                std::iter::repeat(BitValue::V0).take(32).collect::<Vec<_>>(),
                vec![BitValue::V0, BitValue::V0, BitValue::V1, BitValue::V1],
            ]
            .concat(),
        ),
        Mask(
            vec![
                std::iter::repeat(BitValue::V0).take(32).collect::<Vec<_>>(),
                vec![BitValue::V0, BitValue::V1, BitValue::V1, BitValue::V0],
            ]
            .concat(),
        ),
        Mask(
            vec![
                std::iter::repeat(BitValue::V0).take(32).collect::<Vec<_>>(),
                vec![BitValue::V0, BitValue::V1, BitValue::V1, BitValue::V1],
            ]
            .concat(),
        ),
    ];
    assert_eq!(mask.to_combinations(), expected);
}

impl From<usize> for Mask {
    fn from(item: usize) -> Self {
        Self(
            format!("{:036b}", item as u64)
                .chars()
                .map(|c| match c {
                    '0' => BitValue::V0,
                    '1' => BitValue::V1,
                    _ => BitValue::VX,
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[test]
fn test_mask_from_usize() {
    let mask13: Mask = (13 as usize).into();
    assert_eq!(
        mask13,
        Mask(
            vec![
                std::iter::repeat(BitValue::V0).take(32).collect::<Vec<_>>(),
                vec![BitValue::V1, BitValue::V1, BitValue::V0, BitValue::V1]
            ]
            .concat()
        )
    );
}

impl std::ops::BitOr for Mask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Mask(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|v| match v {
                    (BitValue::VX, _) => BitValue::VX,
                    (BitValue::V1, _) => BitValue::V1,
                    (BitValue::V0, &rhs) => rhs,
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[test]
fn test_mask_bitor() {
    let lhs = Mask(
        vec![
            std::iter::repeat(BitValue::V0).take(30).collect::<Vec<_>>(),
            vec![
                BitValue::V0,
                BitValue::V0,
                BitValue::V1,
                BitValue::V1,
                BitValue::VX,
                BitValue::VX,
            ],
        ]
        .concat(),
    );
    let rhs = Mask(
        vec![
            std::iter::repeat(BitValue::V0).take(30).collect::<Vec<_>>(),
            vec![
                BitValue::V0,
                BitValue::V1,
                BitValue::V0,
                BitValue::V1,
                BitValue::V0,
                BitValue::V1,
            ],
        ]
        .concat(),
    );
    assert_eq!(
        (lhs | rhs),
        Mask(
            vec![
                std::iter::repeat(BitValue::V0).take(30).collect::<Vec<_>>(),
                vec![
                    BitValue::V0,
                    BitValue::V1,
                    BitValue::V1,
                    BitValue::V1,
                    BitValue::VX,
                    BitValue::VX
                ]
            ]
            .concat()
        )
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BitValue {
    VX,
    V0,
    V1,
}

impl BitValue {
    fn to_char(&self) -> Option<char> {
        match self {
            BitValue::VX => None,
            BitValue::V0 => Some('0'),
            BitValue::V1 => Some('1'),
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    Mask(Vec<BitValue>),
    Assignment(usize, usize),
}

impl FromStr for Instr {
    type Err = parser::ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

mod parser {
    use crate::BitValue;
    use crate::Instr;

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Instr, ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Instr
                = m:mask() { m }
                / a:assignment() { a }

            rule mask() -> Instr
                = "mask" _ "=" _ vs:bit_value()+
                {
                    Instr::Mask(vs)
                }

            rule bit_value() -> BitValue
                = "X" { BitValue::VX }
                / "0" { BitValue::V0}
                / "1" { BitValue::V1}

            rule assignment() -> Instr
                = "mem" "[" k:number() "]" _ "=" _ v:number()
                {
                    Instr::Assignment(k, v)
                }

            rule number() -> usize
                = n:$(['0'..='9']+)
                  {
                      n.parse().unwrap()
                  }

            rule _()
                = [' ']+
                / eol()

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}
