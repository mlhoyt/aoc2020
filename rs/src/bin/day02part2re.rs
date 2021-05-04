use regex::Regex;

fn main() {
    let input = include_str!("../../input/day02.txt");
    let values = input
        .lines()
        .map(|line| Password::new(line))
        .collect::<Vec<_>>();

    let result = values
        .iter()
        .filter(|x| match x {
            Some(p) => p.is_valid(),
            None => false,
        })
        .count();
    println!("{:?}", result);
}

#[derive(Debug)]
struct Password {
    rule_min: usize,
    rule_max: usize,
    rule_char: char,
    value: String,
}

impl Password {
    fn new(s: &str) -> Option<Password> {
        let re_password = Regex::new(r"^(\d+)-(\d+)\s+([a-z]):\s+(\S+)$").unwrap();

        if re_password.is_match(s) {
            let elems = re_password.captures(s).unwrap();
            Some(Password {
                rule_min: elems
                    .get(1)
                    .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                rule_max: elems
                    .get(2)
                    .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                rule_char: elems
                    .get(3)
                    .map_or(' ', |m| m.as_str().as_bytes()[0] as char),
                value: elems
                    .get(4)
                    .map_or(String::new(), |m| m.as_str().to_string()),
            })
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        let s = self.value.as_bytes();
        let c1 = s[self.rule_min - 1] as char;
        let c2 = s[self.rule_max - 1] as char;

        (c1 == self.rule_char && c2 != self.rule_char)
            || (c1 != self.rule_char && c2 == self.rule_char)
    }
}
