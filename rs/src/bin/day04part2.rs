use std::str::FromStr;

fn main() {
    let input = include_str!("../../input/day04.txt");

    let result = input
        .split("\n\n")
        .map(|x| x.parse::<Passport>())
        .filter(|x| match x {
            Ok(v) => v.is_valid(),
            Err(_) => false,
        })
        .count();

    println!("{:?}\n", result);
}

#[derive(Debug)]
pub struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: (usize, String),
    eye_color: String,
    hair_color: String,
    passport_id: String,
    country_id: String,
}

impl FromStr for Passport {
    type Err = peg::error::ParseError<peg::str::LineCol>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::Parse(s)
    }
}

impl Passport {
    pub fn new() -> Self {
        Self {
            birth_year: 0,
            issue_year: 0,
            expiration_year: 0,
            height: (0, String::new()),
            hair_color: String::new(),
            eye_color: String::new(),
            passport_id: String::new(),
            country_id: String::new(),
        }
    }

    pub fn new_from_attributes(attrs: Vec<PassportAttribute>) -> Self {
        let mut passport = Passport::new();
        for attr in attrs {
            match attr {
                PassportAttribute::BirthYear(v) => {
                    passport.birth_year = v;
                }
                PassportAttribute::IssueYear(v) => {
                    passport.issue_year = v;
                }
                PassportAttribute::ExpirationYear(v) => {
                    passport.expiration_year = v;
                }
                PassportAttribute::Height(v, u) => {
                    passport.height = (v, u);
                }
                PassportAttribute::HairColor(v) => {
                    passport.hair_color = v;
                }
                PassportAttribute::EyeColor(v) => {
                    passport.eye_color = v;
                }
                PassportAttribute::PassportID(v) => {
                    passport.passport_id = v;
                }
                PassportAttribute::CountryID(v) => {
                    passport.country_id = v;
                }
            }
        }

        passport
    }

    fn is_valid(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        let valid_byr = self.birth_year >= 1920 && self.birth_year <= 2002;

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        let valid_iyr = self.issue_year >= 2010 && self.issue_year <= 2020;

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let valid_eyr = self.expiration_year >= 2020 && self.expiration_year <= 2030;

        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        let valid_hgt = (self.height.1 == "cm" && self.height.0 >= 150 && self.height.0 <= 193)
            || (self.height.1 == "in" && self.height.0 >= 59 && self.height.0 <= 76);

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let valid_hcl = self.hair_color.len() == 6;

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let valid_ecl = self.eye_color != "";

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let valid_pid = self.passport_id.len() == 9;

        valid_byr && valid_iyr && valid_eyr && valid_hgt && valid_hcl && valid_ecl && valid_pid
    }
}

pub enum PassportAttribute {
    BirthYear(usize),
    IssueYear(usize),
    ExpirationYear(usize),
    Height(usize, String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
    CountryID(String),
}

peg::parser! {
    grammar parser() for str {
        pub rule Parse() -> Passport
            = attrs:passport_attribute() ++ space()
              {
                  Passport::new_from_attributes(attrs)
              }

        rule passport_attribute() -> PassportAttribute
            = "byr:" value:number()
              {
                  PassportAttribute::BirthYear(value)
              }
            / "iyr:" value:number()
              {
                  PassportAttribute::IssueYear(value)
              }
            / "eyr:" value:number()
              {
                  PassportAttribute::ExpirationYear(value)
              }
            / "hgt:" value:number() units:string()
              {
                  PassportAttribute::Height(value, units.to_string())
              }
            / "hcl:" value:rgb()
              {
                  PassportAttribute::HairColor(value.to_string())
              }
            / "ecl:" value:color_literal()
              {
                  PassportAttribute::EyeColor(value.to_string())
              }
            / "pid:" value:numeric_string()
              {
                  PassportAttribute::PassportID(value.to_string())
              }
            / "cid:" value:numeric_string()
              {
                  PassportAttribute::CountryID(value.to_string())
              }

        rule number() -> usize
            = n:$(['0'..='9']+)
              {
                  n.parse().unwrap()
              }

        rule numeric_string() -> &'input str
            = s:$(['0'..='9']+)
              {
                  s
              }

        rule string() -> &'input str
            = s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-']+)
              {
                  s
              }

        rule rgb() -> &'input str
            = "#" s:$(['a'..='f' | 'A'..='F' | '0'..='9']+)
              {
                  s
              }

        rule color_literal() -> &'input str
            = s:$("amb") { s }
            / s:$("blu") { s }
            / s:$("brn") { s }
            / s:$("gry") { s }
            / s:$("grn") { s }
            / s:$("hzl") { s }
            / s:$("oth") { s }

        rule space()
            = [' ']+
            / eol()

        rule eol()
            = "\n"
            / "\r"
            / "\r" "\n"
    }
}
