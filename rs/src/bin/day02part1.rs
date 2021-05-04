use std::str::FromStr;

fn main() {
    let input = include_str!("../../input/day02.txt");
    let result = input
        .lines()
        .map(|line| line.parse().unwrap())
        .filter(policy_and_password_is_valid)
        .count();
    println!("{}", result);
}

pub struct Policy {
    range_min: usize,
    range_max: usize,
    character: u8,
}

pub struct PolicyAndPassword {
    policy: Policy,
    password: String,
}

impl FromStr for PolicyAndPassword {
    type Err = peg::error::ParseError<peg::str::LineCol>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::Parse(s)
    }
}

fn policy_and_password_is_valid(x: &PolicyAndPassword) -> bool {
    let count = x.password.matches(x.policy.character as char).count();
    count >= x.policy.range_min && count <= x.policy.range_max
}

peg::parser! {
    grammar parser() for str {
        pub rule Parse() -> PolicyAndPassword
            = policy:policy() space() password:string()
              {
                  PolicyAndPassword{policy, password: password.to_string()}
              }

        rule policy() -> Policy
            = range_min:number() "-" range_max:number() space() character:byte() ":"
              {
                  Policy{ range_min, range_max, character }
              }

        rule number() -> usize
            = n:$(['0'..='9']+)
              {
                  n.parse().unwrap()
              }

        rule byte() -> u8
            = c:$(['a'..='z'])
              {
                  c.as_bytes()[0]
              }

        rule string() -> &'input str
            = s:$([_]+)
              {
                  s
              }

        rule space()
            = [' ']+
    }
}
