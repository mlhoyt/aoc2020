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
        .collect::<Vec<_>>()
        .len();
    println!("{:?}", result);
}

#[derive(Debug)]
struct Password {
    rule_min: usize,
    rule_max: usize,
    rule_char: String,
    value: String,
}

impl Password {
    fn new(s: &str) -> Option<Password> {
        let re_password = Regex::new(r"^(\d+)-(\d+)\s+([a-z]):\s+(\S+)$").unwrap();

        match re_password.is_match(s) {
            true => {
                let elems = re_password.captures(s).unwrap();
                return Some(Password {
                    rule_min: elems
                        .get(1)
                        .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                    rule_max: elems
                        .get(2)
                        .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                    rule_char: elems
                        .get(3)
                        .map_or(String::new(), |m| m.as_str().to_string()),
                    value: elems
                        .get(4)
                        .map_or(String::new(), |m| m.as_str().to_string()),
                });
            }
            _ => return None,
        }
    }

    fn is_valid(&self) -> bool {
        let n = self.value.matches(self.rule_char.as_str()).count();
        n >= self.rule_min && n <= self.rule_max
    }
}
