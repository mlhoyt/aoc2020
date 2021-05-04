use std::str::FromStr;

fn main() {
    let input = include_str!("../../input/day02.txt");
    let result = input
        .lines()
        .map(|line| line.parse::<PolicyAndPassword>().unwrap())
        .filter(|x| x.is_valid())
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

impl PolicyAndPassword {
    fn is_valid(&self) -> bool {
        let c = self.policy.character;
        let p = self.password.as_bytes();
        let p_c1 = p[self.policy.range_min - 1];
        let p_c2 = p[self.policy.range_max - 1];

        // (p_c1 == c && p_c2 != c) || (p_c1 != c && p_c2 == c)
        (p_c1 == c) != (p_c2 == c)
    }
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
