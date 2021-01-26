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
        let unset = "".to_string();
        if self.birth_year == unset {
            // println!("birth_year={:?} - {:?}", self.birth_year, self);
            return false;
        } else if self.expiration_year == unset {
            // println!("expiration_year={:?} - {:?}", self.expiration_year, self);
            return false;
        } else if self.eye_color == unset {
            // println!("eye_color={:?} - {:?}", self.eye_color, self);
            return false;
        } else if self.hair_color == unset {
            // println!("hair_color={:?} - {:?}", self.hair_color, self);
            return false;
        } else if self.height == unset {
            // println!("height={:?} - {:?}", self.height, self);
            return false;
        } else if self.issue_year == unset {
            // println!("issue_year={:?} - {:?}", self.issue_year, self);
            return false;
        } else if self.passport_id == unset {
            // println!("passport_id={:?} - {:?}", self.passport_id, self);
            return false;
        } else {
            // println!("success - {:?}", self);
            return true;
        }
    }
}
