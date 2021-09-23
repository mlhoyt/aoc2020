use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// const FILE_NAME: &str = "input/day16.test.txt";
const FILE_NAME: &str = "input/day16.txt";

fn main() {
    let file = File::open(FILE_NAME).expect("cannot open file");
    let mut file = BufReader::new(file);
    let mut input = String::new();
    while file.read_line(&mut input).unwrap() > 0 {}
    // println!("{}", input);

    let data: Data = input.parse().expect("failed to parse input data");
    // println!("{:?}", data);

    let result: usize = data
        .nearby_tickets
        .iter()
        .map(|vs| {
            vs.iter()
                .map(|&v| {
                    let valid = data
                        .rules
                        .iter()
                        .map(|(_, (r1, r2))| (v >= r1.0 && v <= r1.1) || (v >= r2.0 && v <= r2.1))
                        .any(|v| v == true);

                    match valid {
                        false => v,
                        _ => 0,
                    }
                })
                .sum::<usize>()
        })
        .sum();
    println!("{}", result);
}

#[derive(Debug)]
pub struct Data {
    rules: HashMap<String, ((usize, usize), (usize, usize))>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl FromStr for Data {
    type Err = parser::ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

mod parser {
    use crate::Data;
    use std::collections::HashMap;

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Data, ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Data
                = rs:field_rules()
                  yt:your_ticket()
                  nt:nearby_tickets()
                  {
                      let mut rules = HashMap::new();
                      rs.into_iter().for_each(|(k, v)| {rules.insert(k, v);});

                      Data{
                          rules,
                          your_ticket: yt,
                          nearby_tickets: nt,
                      }
                  }

            rule field_rules() -> Vec<(String, ((usize,usize),(usize,usize)))>
                = rs:field_rule()+ eol()
                {
                    rs
                }

            rule field_rule() -> (String, ((usize,usize),(usize,usize)))
                = desc1:word() _ desc2:word() ":" _ r1:value_range() _ "or" _ r2:value_range() eol()
                {
                    (format!("{} {}", desc1, desc2), (r1, r2))
                }
                / desc:word() ":" _ r1:value_range() _ "or" _ r2:value_range() eol()
                {
                    (desc, (r1, r2))
                }

            rule value_range() -> (usize, usize)
                = l:number() "-" u:number()
                {
                    (l, u)
                }

            rule your_ticket() -> Vec<usize>
                = "your ticket:" eol() ns:number_csv() eol() eol()
                {
                    ns
                }

            rule nearby_tickets() -> Vec<Vec<usize>>
                = "nearby tickets:" eol() ns:number_csv() ++ eol()
                {
                    ns
                }

            rule number_csv() -> Vec<usize>
                = ns:number() ++ ","
                {
                      ns
                }

            rule word() -> String
                = n:$(['a'..='z']+)
                  {
                      n.to_string()
                  }

            rule number() -> usize
                = n:$(['0'..='9']+)
                  {
                      n.parse().unwrap()
                  }

            rule _()
                = [' ']+

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}
