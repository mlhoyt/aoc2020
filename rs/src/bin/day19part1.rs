use aoc;
use std::collections::HashMap;

// const FILE_NAME: &str = "input/day19.test.txt";
const FILE_NAME: &str = "input/day19.txt";

fn main() {
    let input = aoc::read_file(FILE_NAME).expect("cannot read file");
    // println!("{}", input);

    let (rules, messages) = parser::parse(&input).expect("cannot parse input");
    // println!("rules:\n{:?}", rules);
    // println!("messages:\n{:?}", messages);

    let re = rules_into_regex(&rules).expect("unable to build regex");
    // println!("re:\n{:?}", re);

    let result = messages.into_iter().filter(|v| re.is_match(v)).count();
    println!("{}", result);
}

mod parser {
    use crate::{Message, Messages, Rule, Rules};

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<(Rules, Messages), ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> (Rules, Messages)
                = rs:grammar_rule() ++ eol() eol()
                  eol()
                  ms:message() ++ eol()
                {
                    let rules = rs.into_iter().collect::<Rules>();

                    (rules, ms)
                }

            rule grammar_rule() -> (usize, Rule)
                = id:number() ":" _ args:number_list() ++ " | "
                {
                    if args.len() == 1 {
                        (id, Rule::Seq(args[0].to_owned()))
                    } else {
                        (id, Rule::Or(args[0].to_owned(), args[1].to_owned()))
                    }
                }
                / id:number() ":" _ "\"" c:$(['a'..='z']) "\""
                {
                    (id, Rule::Literal(c.chars().next().unwrap()))
                }

            rule message() -> Message
                = ms:$(['a'..='z']+)
                {
                    String::from(ms)
                }

            rule number_list() -> Vec<usize>
                = ns:number() ++ " "
                {
                    ns
                }

            rule number() -> usize
                = ns:$(['0'..='9']+)
                {
                    ns.parse().unwrap()
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

type Rules = HashMap<usize, Rule>;

#[derive(Debug)]
pub enum Rule {
    Seq(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
    Literal(char),
}

pub type Messages = Vec<Message>;

pub type Message = String;

fn rules_into_regex(rules: &Rules) -> Result<regex::Regex, regex::Error> {
    regex::Regex::new(&format!("^{}$", rule_into_regex(rules, 0)))
}

fn rule_into_regex(rules: &Rules, rule_key: usize) -> String {
    match rules.get(&rule_key).expect("rule does not exist") {
        Rule::Seq(vs) => {
            let re_string = rule_keys_into_regex(rules, vs);

            format!("(?:{})", re_string)
        }
        Rule::Or(op1, op2) => {
            format!(
                "(?:{}|{})",
                rule_keys_into_regex(rules, op1),
                rule_keys_into_regex(rules, op2)
            )
        }
        Rule::Literal(c) => c.to_string(),
    }
}

fn rule_keys_into_regex(rules: &Rules, keys: &[usize]) -> String {
    keys.iter()
        .map(|v| rule_into_regex(rules, *v))
        .collect::<Vec<_>>()
        .join("")
}
