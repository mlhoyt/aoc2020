use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// const FILE_NAME: &str = "input/day14.test.txt";
const FILE_NAME: &str = "input/day14.txt";

pub fn main() {
    let file = File::open(FILE_NAME).expect("Cannot open file.");
    let file = BufReader::new(file);

    let instrs: Vec<_> = file
        .lines()
        .map(|l| l.expect("Cannot read line."))
        .map(|s| s.parse::<Instr>().expect("Failed to parse instruction."))
        .collect();

    let mut mask = (std::usize::MAX, 0);

    let result = instrs
        .into_iter()
        .map(|v| match v {
            Instr::Mask(a, o) => {
                mask = (a, o);
                None
            }
            Instr::Assignment(k, v) => Some((k, v & mask.0 | mask.1)),
        })
        .filter_map(|v| v)
        .collect::<HashMap<_, _>>()
        .iter()
        .fold(0, |acc, (_, v)| acc + v);

    println!("{:?}", result);
}

#[derive(Debug)]
pub enum Instr {
    Mask(usize, usize),
    Assignment(usize, usize),
}

impl FromStr for Instr {
    type Err = parser::ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

mod parser {
    use crate::Instr;

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Instr, ParseErr> {
        parser::parse(s)
    }

    enum BitValue {
        VX,
        V0,
        V1,
    }

    impl BitValue {
        fn to_and_value(&self) -> char {
            match self {
                BitValue::VX => '1',
                BitValue::V0 => '0',
                BitValue::V1 => '1',
            }
        }

        fn to_or_value(&self) -> char {
            match self {
                BitValue::VX => '0',
                BitValue::V0 => '0',
                BitValue::V1 => '1',
            }
        }
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Instr
                = m:mask() { m }
                / a:assignment() { a }

            rule mask() -> Instr
                = "mask" _ "=" _ v:bit_value()+
                {
                    let and_mask_str = v.iter().map(|v| v.to_and_value()).collect::<String>();
                    let and_mask = usize::from_str_radix(&and_mask_str, 2).expect("Failed to parse mask AND value.");

                    let or_mask_str = v.iter().map(|v| v.to_or_value()).collect::<String>();
                    let or_mask = usize::from_str_radix(&or_mask_str, 2).expect("Failed to parse mask OR value.");

                    Instr::Mask(and_mask, or_mask)
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
