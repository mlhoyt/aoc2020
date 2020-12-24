use regex::Regex;

fn main() {
    let input = include_str!("../../input/day04.txt");
    let attrs = input
        .lines()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .concat()
        .iter()
        .map(|kv| new_attribute(kv))
        .flatten()
        .collect::<Vec<_>>();

    // NOTE: Have to perform split separately because:
    // E0716: temporary value dropped while borrowed consider using a `let` binding to create a longer lived value
    let passports = attrs
        .split(|a| *a == Attribute::EndOfRecord)
        .map(|xs| Passport::new(xs))
        .collect::<Vec<_>>();
    println!("{:?} - {:?}\n", passports.len(), passports);

    let result = passports
        .iter()
        .filter(|p| p.is_valid())
        .collect::<Vec<_>>();
    println!("{:?} - {:?}\n", result.len(), result);
}

#[derive(Debug, PartialEq)]
enum Attribute {
    BirthYear(String),      // byr
    CountryID(String),      // cid
    ExpirationYear(String), // eyr
    EyeColor(String),       // ecl
    HairColor(String),      // hcl
    Height(String),         // hgt
    IssueYear(String),      // iyr
    PassportID(String),     // pid
    Unknown(String, String),
    EndOfRecord,
}

fn new_attribute(kv: &str) -> Option<Attribute> {
    if kv == "" {
        return Some(Attribute::EndOfRecord);
    }

    let fields = kv.split(":").collect::<Vec<_>>();

    if fields.len() != 2 {
        return None;
    }

    match fields[0] {
        "byr" => Some(Attribute::BirthYear(fields[1].to_string())),
        "iyr" => Some(Attribute::IssueYear(fields[1].to_string())),
        "eyr" => Some(Attribute::ExpirationYear(fields[1].to_string())),
        "hgt" => Some(Attribute::Height(fields[1].to_string())),
        "hcl" => Some(Attribute::HairColor(fields[1].to_string())),
        "ecl" => Some(Attribute::EyeColor(fields[1].to_string())),
        "pid" => Some(Attribute::PassportID(fields[1].to_string())),
        "cid" => Some(Attribute::CountryID(fields[1].to_string())),
        _ => Some(Attribute::Unknown(
            fields[0].to_string(),
            fields[1].to_string(),
        )),
    }
}

#[derive(Debug)]
struct Passport {
    birth_year: String,
    country_id: String,
    expiration_year: String,
    eye_color: String,
    hair_color: String,
    height: String,
    issue_year: String,
    passport_id: String,
}

impl Passport {
    fn init() -> Passport {
        Passport {
            birth_year: "".to_string(),
            country_id: "".to_string(),
            expiration_year: "".to_string(),
            eye_color: "".to_string(),
            hair_color: "".to_string(),
            height: "".to_string(),
            issue_year: "".to_string(),
            passport_id: "".to_string(),
        }
    }

    fn new(xs: &[Attribute]) -> Passport {
        let mut passport = Passport::init();

        for x in xs {
            match x {
                Attribute::BirthYear(s) => {
                    passport.birth_year = s.clone();
                }
                Attribute::IssueYear(s) => {
                    passport.issue_year = s.clone();
                }
                Attribute::ExpirationYear(s) => {
                    passport.expiration_year = s.clone();
                }
                Attribute::Height(s) => {
                    passport.height = s.clone();
                }
                Attribute::HairColor(s) => {
                    passport.hair_color = s.clone();
                }
                Attribute::EyeColor(s) => {
                    passport.eye_color = s.clone();
                }
                Attribute::PassportID(s) => {
                    passport.passport_id = s.clone();
                }
                Attribute::CountryID(s) => {
                    passport.country_id = s.clone();
                }
                _ => {}
            }
        }

        passport
    }

    fn is_valid(&self) -> bool {
        return self.validate_birth_year()
            && self.validate_country_id()
            && self.validate_expiration_year()
            && self.validate_eye_color()
            && self.validate_hair_color()
            && self.validate_height()
            && self.validate_issue_year()
            && self.validate_passport_id();
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002
    fn validate_birth_year(&self) -> bool {
        let v = &self.birth_year;
        let min: i32 = 1920;
        let max: i32 = 2002;

        if v.as_str() == "" {
            return false;
        } else if v.len() != 4 {
            return false;
        } else {
            match v.parse::<i32>() {
                Err(_) => return false,
                Ok(n) => {
                    if n < min || n > max {
                        return false;
                    } else {
                        return true;
                    }
                }
            }
        }
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn validate_issue_year(&self) -> bool {
        let v = &self.issue_year;
        let min: i32 = 2010;
        let max: i32 = 2020;

        if v.as_str() == "" {
            return false;
        } else if v.len() != 4 {
            return false;
        } else {
            match v.parse::<i32>() {
                Err(_) => return false,
                Ok(n) => {
                    if n < min || n > max {
                        return false;
                    } else {
                        return true;
                    }
                }
            }
        }
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn validate_expiration_year(&self) -> bool {
        let v = &self.expiration_year;
        let min: i32 = 2020;
        let max: i32 = 2030;

        if v.as_str() == "" {
            return false;
        } else if v.len() != 4 {
            return false;
        } else {
            match v.parse::<i32>() {
                Err(_) => return false,
                Ok(n) => {
                    if n < min || n > max {
                        return false;
                    } else {
                        return true;
                    }
                }
            }
        }
    }

    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    fn validate_height(&self) -> bool {
        let v = &self.height;
        let min_cm: i32 = 150;
        let max_cm: i32 = 193;
        let min_in: i32 = 59;
        let max_in: i32 = 76;

        if v.as_str() == "" {
            return false;
        } else {
            let re_height = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            if !re_height.is_match(v) {
                return false;
            } else {
                let elems = re_height.captures(v).unwrap();
                let v_num = elems
                    .get(1)
                    .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
                let v_unit = elems.get(2).map_or(" ", |m| m.as_str());
                match v_unit {
                    "cm" => {
                        return v_num >= min_cm && v_num <= max_cm;
                    }
                    "in" => {
                        return v_num >= min_in && v_num <= max_in;
                    }
                    _ => return false,
                }
            }
        }
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn validate_hair_color(&self) -> bool {
        let v = &self.hair_color;

        if v.as_str() == "" {
            return false;
        } else {
            let re_hair_color = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            return re_hair_color.is_match(v);
        }
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn validate_eye_color(&self) -> bool {
        let v = &self.eye_color;
        let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        if v.as_str() == "" {
            return false;
        } else {
            return valid_colors.iter().any(|&x| x == v.as_str());
        }
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn validate_passport_id(&self) -> bool {
        let v = &self.passport_id;
        let len = 9;

        if v.as_str() == "" {
            return false;
        } else if v.len() != len {
            return false;
        } else {
            return true;
        }
    }

    // cid (Country ID) - ignored, missing or not
    fn validate_country_id(&self) -> bool {
        return true;
    }
}
