use std::collections::HashMap;
use std::collections::HashSet;
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

    // Create a list of valid tickets (each ticket is still a list of usize values).
    // Result: [
    //   [usize, ...],
    //   ...
    // ]
    let valid_tickets = data
        .nearby_tickets
        .iter()
        .filter(|vs| {
            !vs.iter()
                .map(|&v| {
                    data.rules
                        .iter()
                        .map(|(_, (r1, r2))| (v >= r1.0 && v <= r1.1) || (v >= r2.0 && v <= r2.1))
                        .any(|v| v == true)
                })
                .any(|v| v == false)
        })
        .collect::<Vec<_>>();

    // Convert each ticket value to a set of rule labels that are valid for the value.
    // Result: [
    //   [HashSet, ...],
    //   ...
    // ]
    let valid_rules = valid_tickets
        .iter()
        .map(|vs| {
            vs.iter()
                .map(|&v| {
                    data.rules
                        .iter()
                        .map(|(l, (r1, r2))| {
                            if (v >= r1.0 && v <= r1.1) || (v >= r2.0 && v <= r2.1) {
                                Some(l)
                            } else {
                                None
                            }
                        })
                        .filter_map(|v| v)
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Create a set of all the rule labels.
    // Result: HashSet => {String, ...}
    let all_rule_names = data.rules.iter().map(|(l, _)| l).collect::<HashSet<_>>();

    // Collapse the list of lists of sets of valid rule labels by column via set intersection.
    // Result: [HashSet, ...]
    let possible_rules = valid_rules.iter().fold(
        vec![all_rule_names.clone(); data.your_ticket.len()],
        |xsection, vs| {
            xsection
                .iter()
                .zip(vs)
                .map(|(xs, v)| xs.intersection(v).copied().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
        },
    );

    // Populate the initial list of field-labels with those that reduced to a single value via set
    // intersection.
    // Result: [Option<String>, ...]
    let mut field_labels = possible_rules
        .iter()
        .map(|ls| {
            if ls.len() > 1 {
                None
            } else {
                Some(*(ls.iter().collect::<Vec<_>>())[0])
            }
        })
        .collect::<Vec<_>>();

    // Repeatedly iterate over the remaining unknown field-labels using set difference between the
    // set of valid rules and each of the known field-labels until there are no more unknown
    // field-labels or no unknown field-labels are discovered in an iteration.  This approach is
    // similar to solitaire where if you go through all the cards in your hand and nothing can be
    // placed then you have lost as each successive iteration will yield the same.  It was not
    // known if this approach would work for this problem but it turns out that it does.
    let mut unknowns: usize = field_labels
        .iter()
        .map(|v| match v {
            None => 1,
            _ => 0,
        })
        .sum();
    loop {
        let mut changes = 0;

        possible_rules.iter().enumerate().for_each(|(i, ls)| {
            if field_labels[i].is_none() {
                let diff = field_labels.iter().fold(ls.clone(), |diff, l| match l {
                    Some(l) => {
                        let mut updated = diff.clone();
                        updated.remove(l);
                        updated
                    }
                    _ => diff,
                });

                if diff.len() == 1 {
                    field_labels[i] = Some((diff.iter().collect::<Vec<_>>())[0]);
                    changes += 1;
                }
            }
        });

        unknowns -= changes;
        if changes == 0 || unknowns == 0 {
            break;
        }
    }

    // Check that all field-labels are known
    if field_labels.iter().any(|v| v.is_none()) {
        panic!("at least one field-label is not known!");
    }

    let result: usize = data
        .your_ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            let mut label = "";
            if let Some(v) = field_labels[*i] {
                label = v;
            }

            label.starts_with("departure")
        })
        .map(|(_, &v)| v)
        .product();
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
